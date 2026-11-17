function App() {
  return (
    <main
      className="flex min-h-screen items-center justify-center bg-slate-950 px-6 py-16 text-slate-100"
      data-testid="app-root"
    >
      <section className="w-full max-w-2xl rounded-2xl border border-slate-800 bg-slate-900/80 p-8 shadow-2xl shadow-black/20 sm:p-12">
        <div className="mb-8 inline-flex items-center gap-2 rounded-full border border-sky-400/20 bg-sky-400/10 px-3 py-1 text-xs font-medium tracking-wide text-sky-300">
          <span className="size-2 rounded-full bg-sky-400" />
          FOUNDATION BUILD
        </div>

        <h1 className="text-3xl font-semibold tracking-tight sm:text-4xl">
          DevEnv Manager
        </h1>

        <p className="mt-4 max-w-xl text-base leading-7 text-slate-300">
          The application foundation is in place. Environment inventory and management
          capabilities will be introduced in subsequent milestones.
        </p>

        <dl className="mt-10 grid gap-4 border-t border-slate-800 pt-6 sm:grid-cols-3">
          <div>
            <dt className="text-xs uppercase tracking-wider text-slate-500">Frontend</dt>
            <dd className="mt-2 text-sm font-medium text-slate-200">React + TypeScript</dd>
          </div>
          <div>
            <dt className="text-xs uppercase tracking-wider text-slate-500">Desktop</dt>
            <dd className="mt-2 text-sm font-medium text-slate-200">Tauri 2</dd>
          </div>
          <div>
            <dt className="text-xs uppercase tracking-wider text-slate-500">Backend</dt>
            <dd className="mt-2 text-sm font-medium text-slate-200">Rust</dd>
          </div>
        </dl>
      </section>
    </main>
  );
}

export default App;
