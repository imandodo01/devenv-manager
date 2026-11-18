export type EnvironmentToolId =
  | "node"
  | "php"
  | "java"
  | "rustc"
  | "cargo"
  | "git"
  | "composer";

export type DetectionStatus =
  | "detected"
  | "notFound"
  | "ambiguous"
  | "inspectionError";

export type EnvironmentInstallation = {
  executablePath: string;
  artifactPath: string | null;
  version: string;
};

export type EnvironmentToolResult = {
  tool: EnvironmentToolId;
  label: string;
  status: DetectionStatus;
  installations: EnvironmentInstallation[];
  incomplete: boolean;
  notes: string[];
};

export type EnvironmentInventorySnapshot = {
  /** Unix timestamp in seconds, encoded as a string by the Rust command. */
  scannedAt: string;
  tools: EnvironmentToolResult[];
};

export function getDetectionStatusLabel(status: DetectionStatus): string {
  switch (status) {
    case "detected":
      return "Detected";
    case "notFound":
      return "Not found in approved locations";
    case "ambiguous":
      return "Multiple installations";
    case "inspectionError":
      return "Inspection incomplete";
  }
}
