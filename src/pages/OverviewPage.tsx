import SummaryCard from "../components/SummaryCard";
import StatusBadge from "../components/StatusBadge";

const summaries = [
  {
    title: "Environment inventory",
    description: "Tool and runtime detection is planned for M2.",
    marker: "EN",
  },
  {
    title: "Repositories",
    description: "Repository discovery and registration are planned for a later milestone.",
    marker: "RE",
  },
  {
    title: "Requirements",
    description: "Project requirement inspection is not implemented yet.",
    marker: "RQ",
  },
];

function OverviewPage() {
  return (
    <div className="space-y-6" data-testid="overview-page">
      <section className="relative overflow-hidden rounded-2xl border border-slate-800 bg-gradient-to-br from-slate-900 via-slate-900 to-slate-950 p-6 shadow-xl shadow-black/10 sm:p-8">
        <div
          aria-hidden="true"
          className="pointer-events-none absolute -right-16 -top-24 size-72 rounded-full border border-sky-400/10"
        />
        <div
          aria-hidden="true"
          className="pointer-events-none absolute -right-4 -top-12 size-48 rounded-full border border-sky-400/10"
        />
        <div className="relative max-w-2xl">
          <div className="mb-5 inline-flex items-center gap-2 rounded-full border border-sky-400/20 bg-sky-400/10 px-3 py-1.5 text-[10px] font-semibold uppercase tracking-[0.16em] text-sky-300">
            <span className="size-1.5 rounded-full bg-sky-300" />
            Foundation ready
          </div>
          <h2 className="text-2xl font-semibold leading-tight tracking-tight text-white sm:text-3xl">
            Your development environment, in one place.
          </h2>
          <p className="mt-4 max-w-xl text-sm leading-7 text-slate-400">
            DevEnv Manager is being built to help you understand your local tools,
            runtimes, and project requirements. The application shell is ready; environment
            data will appear as those capabilities are implemented.
          </p>
        </div>
      </section>

      <section aria-labelledby="workspace-summary-heading">
        <div className="mb-4 flex items-center justify-between gap-4">
          <div>
            <h2
              id="workspace-summary-heading"
              className="text-sm font-semibold text-slate-200"
            >
              Workspace summary
            </h2>
            <p className="mt-1 text-xs text-slate-500">
              Capability availability, not a live inventory report.
            </p>
          </div>
          <StatusBadge>Initial setup</StatusBadge>
        </div>

        <div className="grid gap-3 xl:grid-cols-3">
          {summaries.map((item) => (
            <SummaryCard
              key={item.title}
              marker={item.marker}
              title={item.title}
              status="Planned"
              description={item.description}
            />
          ))}
        </div>
      </section>

      <section className="rounded-xl border border-slate-800 bg-slate-900/40 p-5 sm:p-6">
        <div className="flex flex-wrap items-start justify-between gap-3">
          <div>
            <h2 className="text-sm font-semibold text-slate-200">What happens next</h2>
            <p className="mt-1 text-xs leading-5 text-slate-500">
              The next milestone introduces read-only environment inventory and runtime detection.
            </p>
          </div>
          <StatusBadge tone="info">M2 · Environment inventory</StatusBadge>
        </div>
      </section>
    </div>
  );
}

export default OverviewPage;
