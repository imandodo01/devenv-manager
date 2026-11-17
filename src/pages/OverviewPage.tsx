const summaries = [
  {
    label: "Environment inventory",
    state: "Not available yet",
    detail: "Tool and runtime detection is planned for M2.",
    marker: "EN",
  },
  {
    label: "Repositories",
    state: "Not available yet",
    detail: "Repository discovery and registration are planned for a later milestone.",
    marker: "RE",
  },
  {
    label: "Requirements",
    state: "Not available yet",
    detail: "Project requirement inspection is not implemented yet.",
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
          <span className="rounded-md border border-slate-800 bg-slate-900 px-2 py-1 text-[10px] font-medium uppercase tracking-wider text-slate-500">
            Initial setup
          </span>
        </div>

        <div className="grid gap-3 xl:grid-cols-3">
          {summaries.map((item) => (
            <article
              key={item.label}
              className="rounded-xl border border-slate-800 bg-slate-900/60 p-5"
            >
              <div className="mb-5 flex items-center justify-between">
                <span className="flex size-9 items-center justify-center rounded-lg border border-slate-800 bg-slate-950 text-[10px] font-semibold text-slate-400">
                  {item.marker}
                </span>
                <span className="rounded-full border border-slate-700/80 bg-slate-950/70 px-2.5 py-1 text-[10px] font-medium text-slate-400">
                  Planned
                </span>
              </div>
              <h3 className="text-sm font-medium text-slate-200">{item.label}</h3>
              <p className="mt-2 text-sm font-semibold text-slate-300">{item.state}</p>
              <p className="mt-2 text-xs leading-5 text-slate-500">{item.detail}</p>
            </article>
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
          <span className="rounded-md border border-sky-400/20 bg-sky-400/5 px-2.5 py-1 text-[10px] font-semibold uppercase tracking-wider text-sky-300">
            M2 · Environment inventory
          </span>
        </div>
      </section>
    </div>
  );
}

export default OverviewPage;
