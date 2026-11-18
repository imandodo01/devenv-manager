use serde::Serialize;
use std::{
    collections::BTreeMap,
    env,
    ffi::OsString,
    fs,
    io::{Read, ErrorKind},
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
    pub version: String,
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
        let Ok(canonical) = executable.canonicalize() else {
            return;
        };
        if !canonical.is_file() || !has_expected_name(&canonical, tool.executable_name()) {
            return;
        }

        let canonical_artifact = artifact.and_then(|path| path.canonicalize().ok())
            .filter(|path| path.is_file());
        let candidates = self.candidates.entry(tool).or_default();
        let duplicate = candidates.iter().any(|item| {
            item.executable == canonical && item.artifact == canonical_artifact
        });
        if duplicate {
            return;
        }
        if candidates.len() >= MAX_CANDIDATES_PER_TOOL {
            self.note(tool, "Candidate limit reached; additional installations were not inspected.");
            return;
        }
        candidates.push(Candidate {
            executable: canonical,
            artifact: canonical_artifact,
        });
    }
}

fn has_expected_name(path: &Path, expected: &str) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.eq_ignore_ascii_case(expected))
}

fn env_path(name: &str) -> Option<PathBuf> {
    env::var_os(name).filter(|value| !value.is_empty()).map(PathBuf::from)
}

fn add_path_candidates(discovery: &mut Discovery) {
    let Some(path_value) = env::var_os("PATH") else {
        return;
    };

    let path_entries: Vec<_> = env::split_paths(&path_value).take(MAX_PATH_ENTRIES + 1).collect();
    for directory in path_entries.iter().take(MAX_PATH_ENTRIES) {
        if !directory.is_dir() {
            continue;
        }
        for tool in [ToolId::Node, ToolId::Php, ToolId::Java, ToolId::Rustc, ToolId::Cargo, ToolId::Git] {
            discovery.add(tool, directory.join(tool.executable_name()), None);
        }
        let composer = directory.join("composer.phar");
        if composer.is_file() {
            discovery.add(ToolId::Composer, composer, None);
        }
    }
    if path_entries.len() > MAX_PATH_ENTRIES {
        for tool in [ToolId::Node, ToolId::Php, ToolId::Java, ToolId::Rustc, ToolId::Cargo, ToolId::Git, ToolId::Composer] {
            discovery.note(tool, "PATH entry limit reached; remaining entries were not inspected.");
        }
    }
}

fn add_manager_root(
    discovery: &mut Discovery,
    tool: ToolId,
    root: &Path,
    nested_version_directories: bool,
) {
    discovery.add(tool, root.join(tool.executable_name()), None);

    if !nested_version_directories {
        return;
    }

    let entries = match fs::read_dir(root) {
        Ok(entries) => entries,
        Err(error) if error.kind() == ErrorKind::NotFound => return,
        Err(_) => {
            discovery.note(tool, format!("Could not inspect configured directory: {}", root.display()));
            return;
        }
    };

    for (index, entry) in entries.take(MAX_CHILD_DIRECTORIES).enumerate() {
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
        if index + 1 == MAX_CHILD_DIRECTORIES {
            discovery.note(tool, "Version-manager directory limit reached; remaining entries were not inspected.");
        }
    }
}

fn discover() -> Discovery {
    let mut result = Discovery::default();
    add_path_candidates(&mut result);

    if let Some(root) = env_path("NVM_HOME") {
        add_manager_root(&mut result, ToolId::Node, &root, true);
    }
    if let Some(root) = env_path("NVM_SYMLINK") {
        add_manager_root(&mut result, ToolId::Node, &root, false);
    }
    if let Some(root) = env_path("PHP_HOME") {
        add_manager_root(&mut result, ToolId::Php, &root, false);
    }
    if let Some(root) = env_path("JAVA_HOME") {
        add_manager_root(&mut result, ToolId::Java, &root.join("bin"), false);
    }
    if let Some(root) = env_path("CARGO_HOME") {
        add_manager_root(&mut result, ToolId::Rustc, &root.join("bin"), false);
        add_manager_root(&mut result, ToolId::Cargo, &root.join("bin"), false);
    }
    if let Some(root) = env_path("USERPROFILE").map(|path| path.join(".cargo").join("bin")) {
        add_manager_root(&mut result, ToolId::Rustc, &root, false);
        add_manager_root(&mut result, ToolId::Cargo, &root, false);
    }

    if let Some(program_files) = env_path("ProgramFiles") {
        add_manager_root(&mut result, ToolId::Node, &program_files.join("nodejs"), false);
        add_manager_root(&mut result, ToolId::Php, &program_files.join("PHP"), false);
        add_manager_root(&mut result, ToolId::Git, &program_files.join("Git").join("cmd"), false);

        let java_root = program_files.join("Java");
        match fs::read_dir(&java_root) {
            Ok(entries) => {
                for (index, entry) in entries.take(MAX_CHILD_DIRECTORIES).enumerate() {
                    if let Ok(entry) = entry {
                        if entry.file_type().is_ok_and(|kind| kind.is_dir()) {
                            add_manager_root(&mut result, ToolId::Java, &entry.path().join("bin"), false);
                        }
                    }
                    if index + 1 == MAX_CHILD_DIRECTORIES {
                        result.note(ToolId::Java, "Java installation directory limit reached.");
                    }
                }
            }
            Err(error) if error.kind() == ErrorKind::NotFound => {}
            Err(_) => result.note(ToolId::Java, "Could not inspect the conventional Java installation directory."),
        }
    }

    if let Some(program_data) = env_path("ProgramData") {
        let composer = program_data.join("ComposerSetup").join("bin").join("composer.phar");
        if composer.is_file() {
            result.add(ToolId::Composer, composer, None);
        }
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
    let mut child = Command::new(executable)
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

fn parse_version(tool: ToolId, output: &str) -> Option<String> {
    for token in output.split_whitespace() {
        let candidate = token.trim_matches(|character: char| {
            !character.is_ascii_alphanumeric() && !matches!(character, '.' | '+' | '-' | '_')
        }).trim_start_matches('v');

        let has_digit = candidate.chars().any(|character| character.is_ascii_digit());
        let valid = !candidate.is_empty()
            && has_digit
            && candidate.chars().all(|character| {
                character.is_ascii_alphanumeric() || matches!(character, '.' | '+' | '-' | '_')
            });

        if valid && (candidate.contains('.') || candidate.chars().all(|c| c.is_ascii_digit())) {
            return Some(candidate.to_string());
        }
    }

    if tool == ToolId::Java {
        return output.lines().find_map(|line| {
            let lower = line.to_ascii_lowercase();
            if lower.contains("version") {
                line.split('"').nth(1).map(str::to_string)
            } else {
                None
            }
        });
    }
    None
}

fn inspect_candidate(tool: ToolId, candidate: &Candidate, php_runner: Option<&Path>) -> Result<Installation, String> {
    let (program, args): (PathBuf, Vec<OsString>) = if tool == ToolId::Composer {
        let php = php_runner.ok_or_else(|| "Composer requires exactly one validated PHP installation.".to_string())?;
        let artifact = candidate.artifact.as_ref().unwrap_or(&candidate.executable);
        (php.to_path_buf(), vec![artifact.as_os_str().to_owned(), OsString::from("--version")])
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
        return Err("Version command returned a non-zero or unavailable exit code.".to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}\n{stderr}");
    let version = parse_version(tool, &combined)
        .ok_or_else(|| "Version output was not recognized; no version was inferred.".to_string())?;

    Ok(Installation {
        executable_path: if tool == ToolId::Composer {
            program.to_string_lossy().into_owned()
        } else {
            candidate.executable.to_string_lossy().into_owned()
        },
        artifact_path: if tool == ToolId::Composer {
            Some(candidate.artifact.as_ref().unwrap_or(&candidate.executable).to_string_lossy().into_owned())
        } else {
            candidate.artifact.as_ref().map(|path| path.to_string_lossy().into_owned())
        },
        version,
    })
}

fn inspect_tool(tool: ToolId, discovery: &Discovery, php_count: usize, php_path: Option<&Path>) -> ToolResult {
    let candidates = discovery.candidates.get(&tool).cloned().unwrap_or_default();
    let mut notes = discovery.notes.get(&tool).cloned().unwrap_or_default();
    let incomplete = !notes.is_empty();

    if candidates.is_empty() {
        return ToolResult {
            tool,
            label: tool.label(),
            status: if incomplete { "inspectionError" } else { "notFound" },
            installations: Vec::new(),
            incomplete,
            notes,
        };
    }

    if tool == ToolId::Composer && php_count != 1 {
        notes.push(if php_count == 0 {
            "Composer PHAR was found, but no validated PHP installation is available.".to_string()
        } else {
            "Composer PHAR was found, but PHP detection is ambiguous; no PHP runner was selected.".to_string()
        });
        return ToolResult {
            tool,
            label: tool.label(),
            status: "inspectionError",
            installations: Vec::new(),
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
            }
        }
    }

    installations.sort_by(|left, right| left.executable_path.cmp(&right.executable_path));
    let status = if candidates.len() > 1 {
        "ambiguous"
    } else if installations.len() == 1 {
        "detected"
    } else {
        "inspectionError"
    };

    ToolResult {
        tool,
        label: tool.label(),
        status,
        installations,
        incomplete: incomplete || notes.len() > 0,
        notes,
    }
}

pub fn collect_inventory() -> InventorySnapshot {
    let discovery = discover();

    let php_candidates = discovery.candidates.get(&ToolId::Php).cloned().unwrap_or_default();
    let php_path = (php_candidates.len() == 1)
        .then(|| php_candidates[0].executable.clone());
    let php_count = php_candidates.len();

    let tools = [
        ToolId::Node, ToolId::Php, ToolId::Java, ToolId::Rustc,
        ToolId::Cargo, ToolId::Git, ToolId::Composer,
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
        assert_eq!(parse_version(ToolId::Node, "v22.23.4"), Some("22.23.4".into()));
        assert_eq!(parse_version(ToolId::Php, "PHP 8.3.14 (cli)"), Some("8.3.14".into()));
        assert_eq!(parse_version(ToolId::Git, "git version 2.45.1.windows.1"), Some("2.45.1.windows.1".into()));
        assert_eq!(parse_version(ToolId::Java, "openjdk version \"21.0.2\" 2024-01-16"), Some("21.0.2".into()));
    }

    #[test]
    fn rejects_unrecognized_version_output() {
        assert_eq!(parse_version(ToolId::Node, "node is installed"), None);
        assert_eq!(parse_version(ToolId::Php, "PHP development build"), None);
    }

    #[test]
    fn expected_executable_name_is_case_insensitive() {
        assert!(has_expected_name(Path::new(r"C:\Tools\NODE.EXE"), "node.exe"));
        assert!(!has_expected_name(Path::new(r"C:\Tools\node.cmd"), "node.exe"));
    }

    #[test]
    fn output_reader_retains_at_most_the_configured_limit() {
        let bytes = vec![b'x'; MAX_OUTPUT_BYTES + 1024];
        let retained = read_capped(bytes.as_slice());
        assert_eq!(retained.len(), MAX_OUTPUT_BYTES);
    }
}
