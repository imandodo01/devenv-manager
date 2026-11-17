import StatusBadge from "./StatusBadge";

type SummaryCardProps = {
  marker: string;
  title: string;
  status: string;
  description: string;
};

function SummaryCard({ marker, title, status, description }: SummaryCardProps) {
  return (
    <article className="rounded-xl border border-slate-800 bg-slate-900/60 p-5">
      <div className="mb-5 flex items-center justify-between">
        <span className="flex size-9 items-center justify-center rounded-lg border border-slate-800 bg-slate-950 text-[10px] font-semibold text-slate-400">
          {marker}
        </span>
        <StatusBadge>{status}</StatusBadge>
      </div>
      <h3 className="text-sm font-medium text-slate-200">{title}</h3>
      <p className="mt-2 text-sm font-semibold text-slate-300">Not available yet</p>
      <p className="mt-2 text-xs leading-5 text-slate-500">{description}</p>
    </article>
  );
}

export default SummaryCard;
