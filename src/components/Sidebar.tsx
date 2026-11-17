import { navigationItems, type PageId } from "../app/navigation";

type SidebarProps = {
  activePage: PageId;
  onNavigate: (page: PageId) => void;
};

function Sidebar({ activePage, onNavigate }: SidebarProps) {
  return (
    <aside className="app-sidebar flex w-64 shrink-0 flex-col border-r border-slate-800/90 bg-slate-950/80 px-4 py-5">
      <div className="mb-10 flex items-center gap-3 px-2">
        <div className="flex size-10 shrink-0 items-center justify-center rounded-xl border border-sky-400/25 bg-sky-400/10 text-sm font-bold tracking-tight text-sky-300">
          DE
        </div>
        <div className="sidebar-copy min-w-0">
          <p className="truncate text-sm font-semibold tracking-wide text-slate-100">
            DevEnv Manager
          </p>
          <p className="mt-1 text-[10px] font-medium uppercase tracking-[0.2em] text-slate-500">
            Local workspace
          </p>
        </div>
      </div>

      <div className="mb-3 px-3 text-[10px] font-semibold uppercase tracking-[0.2em] text-slate-600">
        Workspace
      </div>
      <nav aria-label="Primary navigation" className="flex flex-col gap-1">
        {navigationItems.map((item) => {
          const isActive = activePage === item.id;

          return (
            <button
              key={item.id}
              type="button"
              aria-current={isActive ? "page" : undefined}
              className={`group flex w-full items-center gap-3 rounded-xl px-3 py-3 text-left text-sm transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-sky-400/70 ${
                isActive
                  ? "bg-sky-400/10 text-sky-200"
                  : "text-slate-400 hover:bg-slate-900 hover:text-slate-100"
              }`}
              onClick={() => onNavigate(item.id)}
            >
              <span
                aria-hidden="true"
                className={`flex size-8 shrink-0 items-center justify-center rounded-lg border text-[10px] font-semibold tracking-wide ${
                  isActive
                    ? "border-sky-400/25 bg-sky-400/10 text-sky-300"
                    : "border-slate-800 bg-slate-900 text-slate-500 group-hover:text-slate-300"
                }`}
              >
                {item.shortLabel}
              </span>
              <span className="sidebar-copy font-medium">{item.label}</span>
              {isActive ? (
                <span className="sidebar-copy ml-auto size-1.5 rounded-full bg-sky-300" />
              ) : null}
            </button>
          );
        })}
      </nav>

      <div className="sidebar-copy mt-auto rounded-xl border border-slate-800/80 bg-slate-900/50 p-3">
        <p className="text-xs font-medium text-slate-300">M1 · Application shell</p>
        <p className="mt-1 text-[11px] leading-5 text-slate-500">
          Navigation is ready. Local environment features are introduced in later milestones.
        </p>
      </div>
    </aside>
  );
}

export default Sidebar;
