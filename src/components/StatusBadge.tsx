import type { ReactNode } from "react";

type StatusBadgeProps = {
  children: ReactNode;
  tone?: "muted" | "info";
};

function StatusBadge({ children, tone = "muted" }: StatusBadgeProps) {
  const toneClasses =
    tone === "info"
      ? "border-sky-400/20 bg-sky-400/5 text-sky-300"
      : "border-slate-700/80 bg-slate-950/70 text-slate-400";

  return (
    <span
      className={`inline-flex items-center rounded-full border px-2.5 py-1 text-[10px] font-medium ${toneClasses}`}
    >
      {children}
    </span>
  );
}

export default StatusBadge;
