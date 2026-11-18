import { useReducer } from "react";
import Sidebar from "./components/Sidebar";
import PageHeader from "./components/PageHeader";
import { initialPage, navigationReducer } from "./app/navigation";
import OverviewPage from "./pages/OverviewPage";
import PlaceholderPage from "./pages/PlaceholderPage";
import EnvironmentPage from "./pages/EnvironmentPage";

function App() {
  const [activePage, navigate] = useReducer(navigationReducer, initialPage);

  return (
    <div
      className="flex min-h-screen bg-slate-950 text-slate-100"
      data-testid="app-root"
    >
      <Sidebar activePage={activePage} onNavigate={navigate} />

      <div className="flex min-w-0 flex-1 flex-col">
        <main className="mx-auto flex w-full max-w-[1600px] flex-1 flex-col px-6 py-7 sm:px-8 sm:py-9">
          <PageHeader pageId={activePage} />
          <div className="flex-1">
            {activePage === "overview" ? (
              <OverviewPage />
            ) : activePage === "environment" ? (
              <EnvironmentPage />
            ) : (
              <PlaceholderPage pageId={activePage} />
            )}
          </div>
          <footer className="mt-10 flex flex-wrap items-center justify-between gap-2 border-t border-slate-800/70 pt-4 text-[10px] font-medium uppercase tracking-[0.16em] text-slate-600">
            <span>DevEnv Manager</span>
            <span>Local desktop workspace · M2</span>
          </footer>
        </main>
      </div>
    </div>
  );
}

export default App;
