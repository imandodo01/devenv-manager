import { getPageDefinition, type PageId } from "../app/navigation";\nimport StatusBadge from "../components/StatusBadge";

type PlaceholderPageProps = {
  pageId: Exclude<PageId, "overview">;
};

const milestoneByPage: Record<Exclude<PageId, "overview">, string> = {
  environment: "M2 · Environment inventory",
  repositories: "Later milestone",
  requirements: "M5 · Requirement inspector",
  worklist: "Later milestone",
};

function PlaceholderPage({ pageId }: PlaceholderPageProps) {
  const page = getPageDefinition(pageId);

  return (
    <section
      className="rounded-2xl border border-dashed border-slate-700 bg-slate-900/40 p-6 sm:p-8"
      data-testid={`${pageId}-placeholder`}
    >
      <div className="flex size-11 items-center justify-center rounded-xl border border-slate-700 bg-slate-950 text-xs font-semibold tracking-wide text-slate-400">
        {page.shortLabel}
      </div>
      <p className="mt-6 text-[10px] font-semibold uppercase tracking-[0.2em] text-sky-400/80">
        Capability planned
      </p>
      <h2 className="mt-2 text-xl font-semibold tracking-tight text-slate-100">
        {page.label}
      </h2>
      <p className="mt-3 max-w-xl text-sm leading-7 text-slate-400">
        {page.description} This area is a navigation placeholder. No local data has been
        scanned or loaded, and no system changes have been made.
      </p>
      <div className="mt-7 inline-flex items-center gap-2 rounded-lg border border-slate-800 bg-slate-950/70 px-3 py-2 text-xs font-medium text-slate-400">
        <span className="size-1.5 rounded-full bg-slate-500" />
        {milestoneByPage[pageId]}
      </div>
    </section>
  );
}

export default PlaceholderPage;
