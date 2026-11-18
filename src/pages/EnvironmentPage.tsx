import { useCallback, useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import StatusBadge from "../components/StatusBadge";
import {
  getDetectionStatusLabel,
  type EnvironmentInventorySnapshot,
  type EnvironmentToolResult,
} from "../lib/environment";

type EnvironmentContentProps = {
  snapshot: EnvironmentInventorySnapshot | null;
  loading: boolean;
  error: string | null;
  onRefresh: () => void;
};

function ToolCard({ tool }: { tool: EnvironmentToolResult }) {
  const tone = tool.status === "detected" ? "info" : "muted";

  return (
    <article className="rounded-xl border border-slate-800 bg-slate-900/60 p-5">
      <div className="flex flex-wrap items-start justify-between gap-3">
        <div>
          <h3 className="text-sm font-semibold text-slate-100">{tool.label}</h3>
          <p className="mt-1 text-xs text-slate-500">{tool.tool}</p>
        </div>
        <StatusBadge tone={tone}>{getDetectionStatusLabel(tool.status)}</StatusBadge>
      </div>

      {tool.installations.length > 0 ? (
        <ul className="mt-4 space-y-3">
          {tool.installations.map((installation) => (
            <li
              className="rounded-lg border border-slate-800/80 bg-slate-950/60 p-3"
              key={installation.executablePath + (installation.artifactPath ?? "")}
            >
              <p className="text-sm font-medium text-slate-200">
                {installation.version ?? "Version unavailable"}
              </p>
              {installation.executablePath && (
                <p className="mt-1 break-all font-mono text-[11px] leading-5 text-slate-400">
                  {installation.executablePath}
                </p>
              )}
              {installation.artifactPath && (
                <p className="mt-1 break-all font-mono text-[11px] leading-5 text-slate-500">
                  PHAR: {installation.artifactPath}
                </p>
              )}
            </li>
          ))}
        </ul>
      ) : (
        <p className="mt-4 text-xs leading-6 text-slate-400">
          {tool.status === "notFound"
            ? "No installation was found in the approved detection sources."
            : "No installation with usable version metadata could be confirmed."}
        </p>
      )}

      {tool.incomplete && (
        <p className="mt-3 text-xs text-amber-300">
          Inspection was incomplete. Results may omit additional installations.
        </p>
      )}
      {tool.notes.length > 0 && (
        <ul className="mt-2 list-inside list-disc space-y-1 text-xs leading-5 text-slate-500">
          {tool.notes.map((note) => <li key={note}>{note}</li>)}
        </ul>
      )}
    </article>
  );
}

export function EnvironmentContent({
  snapshot,
  loading,
  error,
  onRefresh,
}: EnvironmentContentProps) {
  return (
    <section className="space-y-6" data-testid="environment-page">
      <div className="flex flex-wrap items-end justify-between gap-4">
        <div>
          <p className="text-xs font-semibold uppercase tracking-[0.16em] text-sky-300">
            Read-only inventory
          </p>
          <h2 className="mt-2 text-2xl font-semibold tracking-tight text-slate-100">
            Environment inventory
          </h2>
          <p className="mt-2 max-w-2xl text-sm leading-6 text-slate-400">
            Supported tools discovered from the current process PATH and approved
            Windows locations. DEM does not change environment configuration.
          </p>
        </div>
        <button
          className="rounded-lg border border-slate-700 bg-slate-900 px-4 py-2 text-xs font-semibold text-slate-200 transition hover:border-slate-500 disabled:cursor-not-allowed disabled:opacity-50"
          disabled={loading}
          onClick={onRefresh}
          type="button"
        >
          {loading ? "Scanning…" : "Scan again"}
        </button>
      </div>

      {error && (
        <div
          className="rounded-xl border border-rose-400/20 bg-rose-400/5 p-4 text-sm text-rose-200"
          role="alert"
        >
          Environment inventory could not be loaded: {error}
        </div>
      )}

      {loading && !snapshot && (
        <p className="rounded-xl border border-slate-800 bg-slate-900/50 p-5 text-sm text-slate-400" role="status">
          Inspecting approved locations and reading tool versions…
        </p>
      )}

      {snapshot && (
        <>
          <div className="flex flex-wrap items-center justify-between gap-2 text-xs text-slate-500">
            <span>{snapshot.tools.length} supported tool entries</span>
            <span>
              Last scan: {Number.isFinite(Number(snapshot.scannedAt))
                ? new Date(Number(snapshot.scannedAt) * 1000).toLocaleString()
                : "time unavailable"}
            </span>
          </div>
          <div className="grid gap-4 xl:grid-cols-2">
            {snapshot.tools.map((tool) => <ToolCard key={tool.tool} tool={tool} />)}
          </div>
        </>
      )}

      <p className="text-xs leading-5 text-slate-600">
        Detection is best-effort and limited to documented sources. “Not found”
        does not prove that a tool is absent from the entire computer. Version
        commands execute local binaries with fixed arguments and bounded execution time. Keep your PATH and configured tool roots trusted.
      </p>
    </section>
  );
}

function EnvironmentPage() {
  const [snapshot, setSnapshot] = useState<EnvironmentInventorySnapshot | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const initialScanStarted = useRef(false);

  const scan = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const result = await invoke<EnvironmentInventorySnapshot>(
        "get_environment_inventory",
      );
      setSnapshot(result);
    } catch (cause) {
      setError(
        cause instanceof Error
          ? cause.message
          : typeof cause === "string"
            ? cause
            : "Unknown command error.",
      );
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    if (initialScanStarted.current) {
      return;
    }
    initialScanStarted.current = true;
    void scan();
  }, [scan]);

  return (
    <EnvironmentContent
      error={error}
      loading={loading}
      onRefresh={() => void scan()}
      snapshot={snapshot}
    />
  );
}

export default EnvironmentPage;
