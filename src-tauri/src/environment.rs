use serde::Serialize;
use std::{
    collections::BTreeMap,
    env,
    ffi::OsString,
    fs,
    io::{ErrorKind, Read},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

const MAX_PATH_ENTRIES: usize = 64;
const MAX_CHILD_DIRECTORIES: usize = 32;
const MAX_CANDIDATES_PER_TOOL: usize = 8;
const COMMAND_TIMEOUT: Duration = Duration::from_secs(3);
const MAX_OUTPUT_BYTES: usize = 8 * 1024;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ToolId {
    Node,
    Php,
    Java,
    Rustc,
    Cargo,
    Git,
    Composer,
}

impl ToolId {
    fn executable_name(self) -> &'static str {
        match self {
            Self::Node => "node.exe",
            Self::Php => "php.exe",
            Self::Java => "java.exe",
            Self::Rustc => "rustc.exe",
            Self::Cargo => "cargo.exe",
            Self::Git => "git.exe",
            Self::Composer => "composer.phar",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Node => "Node.js",
            Self::Php => "PHP",
            Self::Java => "Java",
            Self::Rustc => "Rust compiler",
            Self::Cargo => "Cargo",
            Self::Git => "Git",
            Self::Composer => "Composer",
        }
    }

    fn arguments(self) -> &'static [&'static str] {
        match self {
            Self::Node | Self::Rustc | Self::Cargo => &["--version"],
            Self::Php => &["--version"],
            Self::Java => &["-version"],
            Self::Git => &["--version"],
            Self::Composer => &[],
        }
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Installation {
    pub executable_path: String,
    pub artifact_path: Option<String>,
    pub version: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolResult {
    pub tool: ToolId,
    pub label: &'static str,
    pub status: &'static str,
    pub installations: Vec<Installation>,
    pub incomplete: bool,
    pub notes: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InventorySnapshot {
    pub scanned_at: String,
    pub tools: Vec<ToolResult>,
}

#[derive(Clone)]
struct Candidate {
    executable: PathBuf,
    artifact: Option<PathBuf>,
}

#[derive(Default)]
struct Discovery {
    candidates: BTreeMap<ToolId, Vec<Candidate>>,
    notes: BTreeMap<ToolId, Vec<String>>,
}

impl Discovery {
    fn note(&mut self, tool: ToolId, message: impl Into<String>) {
        let message = message.into();
        let notes = self.notes.entry(tool).or_default();
        if !notes.contains(&message) && notes.len() < 8 {
            notes.push(message);
        }
    }

    fn add(&mut self, tool: ToolId, executable: PathBuf, artifact: Option<PathBuf>) {
        let canonical = match executable.canonicalize() {
            Ok(path) => path,
            Err(error) if error.kind() == ErrorKind::NotFound => return,
            Err(_) => {
                self.note(
                    tool,
                    "A candidate executable path could not be accessed or canonicalized.",
                );
                return;
            }
        };
        if !canonical.is_file() || !has_expected_name(&canonical, tool.executable_name()) {
            self.note(tool, "A candidate path failed executable-file validation.");
            return;
        }

        let canonical_artifact = artifact
            .and_then(|path| path.canonicalize().ok())
            .filter(|path| path.is_file());
        let duplicate = self.candidates.get(&tool).is_some_and(|candidates| {
            candidates
                .iter()
                .any(|item| item.executable == canonical && item.artifact == canonical_artifact)
        });
        if duplicate {
            return;
        }
        let at_limit = self
            .candidates
            .get(&tool)
            .is_some_and(|candidates| candidates.len() >= MAX_CANDIDATES_PER_TOOL);
        if at_limit {
            self.note(
                tool,
                "Candidate limit reached; additional installations were not inspected.",
            );
            return;
        }
        let candidates = self.candidates.entry(tool).or_default();
        candidates.push(Candidate {
            executable: canonical,
            artifact: canonical_artifact,
        });
        candidates.sort_by(|left, right| {
            left.executable
                .cmp(&right.executable)
                .then_with(|| left.artifact.cmp(&right.artifact))
        });
    }
}

fn has_expected_name(path: &Path, expected: &str) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.eq_ignore_ascii_case(expected))
}

fn env_path(name: &str) -> Option<PathBuf> {
    env::var_os(name)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
}

fn add_path_candidates(discovery: &mut Discovery) {
    let Some(path_value) = env::var_os("PATH") else {
        for tool in [
            ToolId::Node,
            ToolId::Php,
            ToolId::Java,
            ToolId::Rustc,
            ToolId::Cargo,
            ToolId::Git,
            ToolId::Composer,
        ] {
            discovery.note(tool, "The current process PATH is unavailable.");
        }
        return;
    };

    let path_entries: Vec<_> = env::split_paths(&path_value)
        .take(MAX_PATH_ENTRIES + 1)
        .collect();
    for directory in path_entries.iter().take(MAX_PATH_ENTRIES) {
        match fs::metadata(directory) {
            Ok(metadata) if metadata.is_dir() => {}
            Ok(_) => continue,
            Err(error) if error.kind() == ErrorKind::NotFound => continue,
            Err(_) => {
                for tool in [
                    ToolId::Node,
                    ToolId::Php,
                    ToolId::Java,
                    ToolId::Rustc,
                    ToolId::Cargo,
                    ToolId::Git,
                    ToolId::Composer,
                ] {
                    discovery.note(tool, "A current PATH directory could not be inspected.");
                }
                continue;
            }
        }
        for tool in [
            ToolId::Node,
            ToolId::Php,
            ToolId::Java,
            ToolId::Rustc,
            ToolId::Cargo,
            ToolId::Git,
        ] {
            discovery.add(tool, directory.join(tool.executable_name()), None);
        }
        let composer = directory.join("composer.phar");
        discovery.add(ToolId::Composer, composer, None);
    }
    if path_entries.len() > MAX_PATH_ENTRIES {
        for tool in [
            ToolId::Node,
            ToolId::Php,
            ToolId::Java,
            ToolId::Rustc,
            ToolId::Cargo,
            ToolId::Git,
            ToolId::Composer,
        ] {
            discovery.note(
                tool,
                "PATH entry limit reached; remaining entries were not inspected.",
            );
        }
    }
}

fn add_manager_root(
    discovery: &mut Discovery,
    tool: ToolId,
    root: &Path,
    nested_version_directories: bool,
) {
    if !nested_version_directories {
        discovery.add(tool, root.join(tool.executable_name()), None);
        return;
    }

    let entries = match fs::read_dir(root) {
        Ok(entries) => entries,
        Err(error) if error.kind() == ErrorKind::NotFound => {
            discovery.note(
                tool,
                "A configured version-manager directory does not exist.",
            );
            return;
        }
        Err(_) => {
            discovery.note(tool, "Could not inspect a configured tool directory.");
            return;
        }
    };

    let entries: Vec<_> = entries.take(MAX_CHILD_DIRECTORIES + 1).collect();
    for entry in entries.iter().take(MAX_CHILD_DIRECTORIES) {
        let Ok(entry) = entry else {
            discovery.note(tool, "A version-manager directory entry could not be read.");
            continue;
        };
        let Ok(file_type) = entry.file_type() else {
            discovery.note(tool, "A version-manager entry could not be inspected.");
            continue;
        };
        if file_type.is_dir() {
            discovery.add(tool, entry.path().join(tool.executable_name()), None);
        }
    }
    if entries.len() > MAX_CHILD_DIRECTORIES {
        discovery.note(
            tool,
            "Version-manager directory limit reached; remaining entries were not inspected.",
        );
    }
}

fn add_configured_root(
    discovery: &mut Discovery,
    tool: ToolId,
    root: &Path,
    nested_version_directories: bool,
) {
    match fs::metadata(root) {
        Ok(metadata) if metadata.is_dir() => {
            add_manager_root(discovery, tool, root, nested_version_directories);
        }
        Ok(_) => discovery.note(tool, "A configured tool path is not a directory."),
        Err(error) if error.kind() == ErrorKind::NotFound => {
            discovery.note(tool, "A configured tool directory does not exist.");
        }
        Err(_) => discovery.note(tool, "Could not inspect a configured tool directory."),
    }
}

fn discover() -> Discovery {
    let mut result = Discovery::default();
    add_path_candidates(&mut result);

    if let Some(root) = env_path("NVM_HOME") {
        add_configured_root(&mut result, ToolId::Node, &root, true);
    }
    if let Some(root) = env_path("NVM_SYMLINK") {
        add_configured_root(&mut result, ToolId::Node, &root, false);
    }
    if let Some(root) = env_path("PHP_HOME") {
        add_configured_root(&mut result, ToolId::Php, &root, false);
    }
    if let Some(root) = env_path("JAVA_HOME") {
        add_configured_root(&mut result, ToolId::Java, &root.join("bin"), false);
    }
    if let Some(root) = env_path("CARGO_HOME") {
        add_configured_root(&mut result, ToolId::Rustc, &root.join("bin"), false);
        add_configured_root(&mut result, ToolId::Cargo, &root.join("bin"), false);
    }
    if let Some(root) = env_path("USERPROFILE").map(|path| path.join(".cargo").join("bin")) {
        add_manager_root(&mut result, ToolId::Rustc, &root, false);
        add_manager_root(&mut result, ToolId::Cargo, &root, false);
    }

    if let Some(program_files) = env_path("ProgramFiles") {
        add_manager_root(
            &mut result,
            ToolId::Node,
            &program_files.join("nodejs"),
            false,
        );
        add_manager_root(&mut result, ToolId::Php, &program_files.join("PHP"), false);
        add_manager_root(
            &mut result,
            ToolId::Git,
            &program_files.join("Git").join("cmd"),
            false,
        );

        let java_root = program_files.join("Java");
        match fs::read_dir(&java_root) {
            Ok(entries) => {
                let entries: Vec<_> = entries.take(MAX_CHILD_DIRECTORIES + 1).collect();
                for entry in entries.iter().take(MAX_CHILD_DIRECTORIES) {
                    let Ok(entry) = entry else {
                        result.note(
                            ToolId::Java,
                            "A Java installation directory entry could not be read.",
                        );
                        continue;
                    };
                    match entry.file_type() {
                        Ok(kind) if kind.is_dir() => add_manager_root(
                            &mut result,
                            ToolId::Java,
                            &entry.path().join("bin"),
                            false,
                        ),
                        Ok(_) => {}
                        Err(_) => result.note(
                            ToolId::Java,
                            "A Java installation directory entry could not be inspected.",
                        ),
                    }
                }
                if entries.len() > MAX_CHILD_DIRECTORIES {
                    result.note(ToolId::Java, "Java installation directory limit reached.");
                }
            }
            Err(error) if error.kind() == ErrorKind::NotFound => {}
            Err(_) => result.note(
                ToolId::Java,
                "Could not inspect the conventional Java installation directory.",
            ),
        }
    }

    if let Some(program_data) = env_path("ProgramData") {
        let composer = program_data
            .join("ComposerSetup")
            .join("bin")
            .join("composer.phar");
        result.add(ToolId::Composer, composer, None);
    }

    result
}

fn read_capped<R: Read>(mut reader: R) -> Vec<u8> {
    let mut retained = Vec::with_capacity(MAX_OUTPUT_BYTES);
    let mut buffer = [0_u8; 1024];
    loop {
        match reader.read(&mut buffer) {
            Ok(0) | Err(_) => break,
            Ok(count) => {
                let remaining = MAX_OUTPUT_BYTES.saturating_sub(retained.len());
                retained.extend_from_slice(&buffer[..count.min(remaining)]);
            }
        }
    }
    retained
}

struct CommandOutput {
    stdout: Vec<u8>,
    stderr: Vec<u8>,
    exit_code: Option<i32>,
    timed_out: bool,
}

fn run_bounded(executable: &Path, arguments: &[OsString]) -> Result<CommandOutput, String> {
    let mut command = Command::new(executable);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000);
    }
    let mut child = command
        .args(arguments)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|_| "Could not start the version command.".to_string())?;

    let stdout = child.stdout.take().expect("piped stdout");
    let stderr = child.stderr.take().expect("piped stderr");
    let stdout_reader = thread::spawn(move || read_capped(stdout));
    let stderr_reader = thread::spawn(move || read_capped(stderr));

    let started = Instant::now();
    let (status, timed_out) = loop {
        match child.try_wait() {
            Ok(Some(status)) => break (Some(status), false),
            Ok(None) if started.elapsed() < COMMAND_TIMEOUT => {
                thread::sleep(Duration::from_millis(20));
            }
            Ok(None) => {
                let _ = child.kill();
                let _ = child.wait();
                break (None, true);
            }
            Err(_) => {
                let _ = child.kill();
                let _ = child.wait();
                break (None, false);
            }
        }
    };

    let stdout = stdout_reader.join().unwrap_or_default();
    let stderr = stderr_reader.join().unwrap_or_default();
    Ok(CommandOutput {
        stdout,
        stderr,
        exit_code: status.and_then(|value| value.code()),
        timed_out,
    })
}

fn parse_version_token(value: &str) -> Option<String> {
    let token = value.split_whitespace().next()?;
    let candidate = token
        .trim_matches(|character: char| {
            !character.is_ascii_alphanumeric() && !matches!(character, '.' | '+' | '-' | '_')
        });
    let has_digit = candidate
        .chars()
        .any(|character| character.is_ascii_digit());
    let valid = !candidate.is_empty()
        && has_digit
        && candidate.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '.' | '+' | '-' | '_')
        });

    valid.then(|| candidate.to_string())
}

fn parse_version(tool: ToolId, output: &str) -> Option<String> {
    for line in output.lines() {
        let line = line.trim();
        let value = match tool {
            ToolId::Node => line
                .strip_prefix('v')
                .filter(|value| {
                    value
                        .chars()
                        .next()
                        .is_some_and(|character| character.is_ascii_digit())
                }),
            ToolId::Php => line.strip_prefix("PHP "),
            ToolId::Java => {
                if line.to_ascii_lowercase().contains("version") {
                    if let Some(start) = line.find('"') {
                        let quoted = &line[start + 1..];
                        if let Some(end) = quoted.find('"') {
                            if let Some(version) = parse_version_token(&quoted[..end]) {
                                return Some(version);
                            }
                        }
                    }
                }
                None
            }
            ToolId::Rustc => line.strip_prefix("rustc "),
            ToolId::Cargo => line.strip_prefix("cargo "),
            ToolId::Git => line.strip_prefix("git version "),
            ToolId::Composer => line.strip_prefix("Composer version "),
        };

        if let Some(value) = value {
            if let Some(version) = parse_version_token(value) {
                return Some(version);
            }
        }
    }

    None
}

fn inspect_candidate(
    tool: ToolId,
    candidate: &Candidate,
    php_runner: Option<&Path>,
) -> Result<Installation, String> {
    let (program, args): (PathBuf, Vec<OsString>) = if tool == ToolId::Composer {
        let php = php_runner.ok_or_else(|| {
            "Composer requires exactly one validated PHP installation.".to_string()
        })?;
        let artifact = candidate.artifact.as_ref().unwrap_or(&candidate.executable);
        (
            php.to_path_buf(),
            vec![
                artifact.as_os_str().to_owned(),
                OsString::from("--no-plugins"),
                OsString::from("--no-scripts"),
                OsString::from("--version"),
            ],
        )
    } else {
        (
            candidate.executable.clone(),
            tool.arguments().iter().map(OsString::from).collect(),
        )
    };

    let output = run_bounded(&program, &args)?;
    if output.timed_out {
        return Err("Version command exceeded the 3-second time limit.".to_string());
    }
    if output.exit_code != Some(0) {
        return Err(
            "Version command returned a non-zero or unavailable exit code.".to_string(),
        );
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}\n{stderr}");
    let version = parse_version(tool, &combined)
        .ok_or_else(|| {
            "Version output was not recognized; no version was inferred.".to_string()
        })?;

    Ok(Installation {
        executable_path: if tool == ToolId::Composer {
            program.to_string_lossy().into_owned()
        } else {
            candidate.executable.to_string_lossy().into_owned()
        },
        artifact_path: if tool == ToolId::Composer {
            Some(
                candidate
                    .artifact
                    .as_ref()
                    .unwrap_or(&candidate.executable)
                    .to_string_lossy()
                    .into_owned(),
            )
        } else {
            candidate
                .artifact
                .as_ref()
                .map(|path| path.to_string_lossy().into_owned())
        },
        version: Some(version),
    })
}

fn detection_status(
    candidate_count: usize,
    usable_version_count: usize,
    has_discovery_errors: bool,
) -> &'static str {
    if candidate_count > 1 {
        "ambiguous"
    } else if candidate_count == 0 {
        if has_discovery_errors {
            "inspectionError"
        } else {
            "notFound"
        }
    } else if usable_version_count == candidate_count {
        "detected"
    } else {
        "inspectionError"
    }
}

fn inspect_tool(
    tool: ToolId,
    discovery: &Discovery,
    php_count: usize,
    php_path: Option<&Path>,
) -> ToolResult {
    let candidates = discovery.candidates.get(&tool).cloned().unwrap_or_default();
    let mut notes = discovery.notes.get(&tool).cloned().unwrap_or_default();
    let incomplete = !notes.is_empty();

    if candidates.is_empty() {
        return ToolResult {
            tool,
            label: tool.label(),
            status: detection_status(0, 0, incomplete),
            installations: Vec::new(),
            incomplete,
            notes,
        };
    }

    if tool == ToolId::Composer && php_count != 1 {
        notes.push(if php_count == 0 {
            "Composer PHAR was found, but no validated PHP installation is available."
                .to_string()
        } else {
            "Composer PHAR was found, but PHP detection is ambiguous; no PHP runner was selected."
                .to_string()
        });
        let installations = candidates
            .iter()
            .map(|candidate| Installation {
                executable_path: php_path
                    .map(|path| path.to_string_lossy().into_owned())
                    .unwrap_or_default(),
                artifact_path: Some(
                    candidate
                        .artifact
                        .as_ref()
                        .unwrap_or(&candidate.executable)
                        .to_string_lossy()
                        .into_owned(),
                ),
                version: None,
            })
            .collect();
        return ToolResult {
            tool,
            label: tool.label(),
            status: detection_status(candidates.len(), 0, true),
            installations,
            incomplete: true,
            notes,
        };
    }

    let mut installations = Vec::new();
    for candidate in &candidates {
        match inspect_candidate(tool, candidate, php_path) {
            Ok(installation) => installations.push(installation),
            Err(error) => {
                if notes.len() < 8 && !notes.contains(&error) {
                    notes.push(error);
                }
                installations.push(Installation {
                    executable_path: if tool == ToolId::Composer {
                        php_path
                            .map(|path| path.to_string_lossy().into_owned())
                            .unwrap_or_default()
                    } else {
                        candidate.executable.to_string_lossy().into_owned()
                    },
                    artifact_path: if tool == ToolId::Composer {
                        Some(
                            candidate
                                .artifact
                                .as_ref()
                                .unwrap_or(&candidate.executable)
                                .to_string_lossy()
                                .into_owned(),
                        )
                    } else {
                        None
                    },
                    version: None,
                });
            }
        }
    }

    installations.sort_by(|left, right| {
        left.executable_path
            .cmp(&right.executable_path)
            .then_with(|| left.artifact_path.cmp(&right.artifact_path))
    });
    let usable_versions = installations
        .iter()
        .filter(|installation| installation.version.is_some())
        .count();
    let status = detection_status(candidates.len(), usable_versions, incomplete);

    ToolResult {
        tool,
        label: tool.label(),
        status,
        installations,
        incomplete: incomplete || !notes.is_empty(),
        notes,
    }
}

pub fn collect_inventory() -> InventorySnapshot {
    let discovery = discover();

    let php_candidates = discovery
        .candidates
        .get(&ToolId::Php)
        .cloned()
        .unwrap_or_default();
    let php_path = (php_candidates.len() == 1).then(|| php_candidates[0].executable.clone());
    let php_count = php_candidates.len();

    let tools = [
        ToolId::Node,
        ToolId::Php,
        ToolId::Java,
        ToolId::Rustc,
        ToolId::Cargo,
        ToolId::Git,
        ToolId::Composer,
    ]
    .into_iter()
    .map(|tool| inspect_tool(tool, &discovery, php_count, php_path.as_deref()))
    .collect();

    InventorySnapshot {
        scanned_at: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
            .to_string(),
        tools,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_supported_version_output_without_guessing() {
        assert_eq!(
            parse_version(ToolId::Node, "v22.23.4"),
            Some("22.23.4".into())
        );
        assert_eq!(
            parse_version(ToolId::Php, "PHP 8.3.14 (cli)"),
            Some("8.3.14".into())
        );
        assert_eq!(
            parse_version(ToolId::Git, "git version 2.45.1.windows.1"),
            Some("2.45.1.windows.1".into())
        );
        assert_eq!(
            parse_version(ToolId::Java, "openjdk version \"21.0.2\" 2024-01-16"),
            Some("21.0.2".into())
        );
        assert_eq!(
            parse_version(ToolId::Java, "openjdk version \"25-ea\" 2026-01-01"),
            Some("25-ea".into())
        );
        assert_eq!(
            parse_version(ToolId::Composer, "Composer version 2.9.1 2025-11-04"),
            Some("2.9.1".into())
        );
    }

    #[test]
    fn rejects_unrecognized_version_output() {
        assert_eq!(parse_version(ToolId::Node, "node is installed"), None);
        assert_eq!(parse_version(ToolId::Php, "PHP development build"), None);
        assert_eq!(parse_version(ToolId::Node, "build 2026"), None);
    }

    #[test]
    fn ignores_unrelated_numeric_output_before_the_tool_version() {
        assert_eq!(
            parse_version(ToolId::Node, "build 2026\nv22.23.4"),
            Some("22.23.4".into())
        );
        assert_eq!(parse_version(ToolId::Node, "build 2026"), None);
    }

    #[test]
    fn detection_status_preserves_missing_ambiguous_and_error_states() {
        assert_eq!(detection_status(0, 0, false), "notFound");
        assert_eq!(detection_status(0, 0, true), "inspectionError");
        assert_eq!(detection_status(2, 0, false), "ambiguous");
        assert_eq!(detection_status(1, 1, false), "detected");
        assert_eq!(detection_status(1, 0, false), "inspectionError");
    }

    #[test]
    fn expected_executable_name_is_case_insensitive() {
        assert!(has_expected_name(Path::new("NODE.EXE"), "node.exe"));
        assert!(!has_expected_name(Path::new("node.cmd"), "node.exe"));
    }

    #[test]
    fn output_reader_retains_at_most_the_configured_limit() {
        let bytes = vec![b'x'; MAX_OUTPUT_BYTES + 1024];
        let retained = read_capped(bytes.as_slice());
        assert_eq!(retained.len(), MAX_OUTPUT_BYTES);
    }

    #[test]
    fn missing_executable_is_reported_as_a_process_start_error() {
        let missing = Path::new("definitely-not-a-real-dem-tool.exe");
        assert!(run_bounded(missing, &[]).is_err());
    }

    #[test]
    #[ignore]
    fn timeout_child_helper() {
        thread::sleep(Duration::from_secs(10));
    }

    #[test]
    #[ignore]
    fn nonzero_exit_child_helper() {
        std::process::exit(7);
    }

    #[test]
    fn timed_out_version_process_is_killed() {
        let executable = std::env::current_exe().expect("current test executable");
        let arguments = [
            OsString::from("--exact"),
            OsString::from("environment::tests::timeout_child_helper"),
            OsString::from("--ignored"),
            OsString::from("--nocapture"),
        ];
        let output = run_bounded(&executable, &arguments).expect("start test helper");
        assert!(output.timed_out);
    }

    #[test]
    fn nonzero_process_exit_is_preserved() {
        let executable = std::env::current_exe().expect("current test executable");
        let arguments = [
            OsString::from("--exact"),
            OsString::from("environment::tests::nonzero_exit_child_helper"),
            OsString::from("--ignored"),
            OsString::from("--nocapture"),
        ];
        let output = run_bounded(&executable, &arguments).expect("start test helper");
        assert_eq!(output.exit_code, Some(7));
    }

    #[test]
    fn missing_candidate_is_not_reported_as_a_discovery_error() {
        let missing = std::env::temp_dir().join(format!(
            "dem-missing-node-{}.exe",
            std::process::id()
        ));
        let mut discovery = Discovery::default();

        discovery.add(ToolId::Node, missing, None);

        assert!(discovery.candidates.get(&ToolId::Node).is_none());
        assert!(discovery.notes.get(&ToolId::Node).is_none());
    }

    #[test]
    fn missing_configured_root_is_reported_as_incomplete() {
        let missing = std::env::temp_dir().join(format!(
            "dem-missing-root-{}",
            std::process::id()
        ));
        let mut discovery = Discovery::default();

        add_configured_root(&mut discovery, ToolId::Node, &missing, false);

        assert!(discovery.candidates.get(&ToolId::Node).is_none());
        assert!(!discovery.notes.get(&ToolId::Node).unwrap().is_empty());
    }

    #[test]
    fn version_manager_root_scans_direct_children_without_using_root_executable() {
        let root = std::env::temp_dir().join(format!(
            "dem-nvm-root-{}",
            std::process::id()
        ));
        let version_directory = root.join("v22.23.4");
        fs::create_dir_all(&version_directory).expect("create version directory");
        fs::write(root.join("node.exe"), b"root fixture").expect("create root fixture");
        fs::write(version_directory.join("node.exe"), b"version fixture")
            .expect("create version fixture");

        let mut discovery = Discovery::default();
        add_manager_root(&mut discovery, ToolId::Node, &root, true);

        let candidates = discovery.candidates.get(&ToolId::Node).unwrap();
        assert_eq!(candidates.len(), 1);
        assert_eq!(
            candidates[0].executable,
            version_directory.join("node.exe").canonicalize().unwrap()
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn candidate_paths_are_sorted_independently_of_discovery_order() {
        let root = std::env::temp_dir().join(format!(
            "dem-sorted-candidates-{}",
            std::process::id()
        ));
        let first_directory = root.join("a");
        let second_directory = root.join("z");
        fs::create_dir_all(&first_directory).expect("create first directory");
        fs::create_dir_all(&second_directory).expect("create second directory");
        let first = first_directory.join("node.exe");
        let second = second_directory.join("node.exe");
        fs::write(&first, b"first fixture").expect("create first fixture");
        fs::write(&second, b"second fixture").expect("create second fixture");

        let mut discovery = Discovery::default();
        discovery.add(ToolId::Node, second, None);
        discovery.add(ToolId::Node, first.clone(), None);

        let candidates = discovery.candidates.get(&ToolId::Node).unwrap();
        assert_eq!(candidates.len(), 2);
        assert_eq!(candidates[0].executable, first.canonicalize().unwrap());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn discovery_deduplicates_canonical_candidate_paths() {
        let directory = std::env::temp_dir().join(format!(
            "dem-m2-candidate-test-{}",
            std::process::id()
        ));
        fs::create_dir_all(&directory).expect("create test directory");
        let executable = directory.join("node.exe");
        fs::write(&executable, b"test fixture").expect("create fixture");

        let mut discovery = Discovery::default();
        discovery.add(ToolId::Node, executable.clone(), None);
        discovery.add(ToolId::Node, executable.clone(), None);

        assert_eq!(discovery.candidates.get(&ToolId::Node).map(Vec::len), Some(1));
        let _ = fs::remove_file(executable);
        let _ = fs::remove_dir(directory);
    }

    #[test]
    fn invalid_candidate_filename_is_reported_as_incomplete() {
        let directory = std::env::temp_dir().join(format!(
            "dem-m2-invalid-candidate-{}",
            std::process::id()
        ));
        fs::create_dir_all(&directory).expect("create test directory");
        let executable = directory.join("not-node.exe");
        fs::write(&executable, b"test fixture").expect("create fixture");

        let mut discovery = Discovery::default();
        discovery.add(ToolId::Node, executable.clone(), None);

        assert!(discovery.candidates.get(&ToolId::Node).is_none());
        assert!(!discovery.notes.get(&ToolId::Node).unwrap().is_empty());
        let _ = fs::remove_file(executable);
        let _ = fs::remove_dir(directory);
    }
}
