import { getPageDefinition, type PageId } from "../app/navigation";

type PageHeaderProps = {
  pageId: PageId;
};

function PageHeader({ pageId }: PageHeaderProps) {
  const page = getPageDefinition(pageId);

  return (
    <header className="mb-8 flex flex-wrap items-end justify-between gap-4 border-b border-slate-800/80 pb-6">
      <div>
        <p className="mb-2 text-[10px] font-semibold uppercase tracking-[0.22em] text-sky-400/80">
          DevEnv Manager / Workspace
        </p>
        <h1 className="text-2xl font-semibold tracking-tight text-slate-100 sm:text-3xl">
          {page.label}
        </h1>
        <p className="mt-2 max-w-2xl text-sm leading-6 text-slate-400">
          {page.description}
        </p>
      </div>
      <div className="inline-flex items-center gap-2 rounded-full border border-slate-700/80 bg-slate-900/70 px-3 py-1.5 text-[11px] font-medium text-slate-400">
        <span className="size-1.5 rounded-full bg-emerald-400" />
        Desktop workspace
      </div>
    </header>
  );
}

export default PageHeader;
