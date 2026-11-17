import { createElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it } from "vitest";
import App from "./App";
import PlaceholderPage from "./pages/PlaceholderPage";
import {
  getPageDefinition,
  navigationItems,
  navigationReducer,
  initialPage,
} from "./app/navigation";

describe("application shell", () => {
  it("renders the overview shell and primary navigation", () => {
    const markup = renderToStaticMarkup(createElement(App));

    expect(markup).toContain('data-testid="app-root"');
    expect(markup).toContain('aria-label="Primary navigation"');
    expect(markup).toContain('aria-current="page"');
    expect(markup).toContain("DevEnv Manager");
    expect(markup).toContain("Workspace summary");
    expect(markup).toContain("Not available yet");
  });

  it("starts on the overview page", () => {
    expect(initialPage).toBe("overview");
    expect(getPageDefinition(initialPage).label).toBe("Overview");
  });

  it("exposes all agreed primary destinations", () => {
    expect(navigationItems.map((item) => item.id)).toEqual([
      "overview",
      "environment",
      "repositories",
      "requirements",
      "worklist",
    ]);
  });

  it("renders each future capability as an explicit placeholder", () => {
    const pages = ["environment", "repositories", "requirements", "worklist"] as const;

    for (const pageId of pages) {
      const markup = renderToStaticMarkup(
        createElement(PlaceholderPage, { pageId }),
      );

      expect(markup).toContain('data-testid="' + pageId + '-placeholder"');
      expect(markup).toContain("Capability planned");
      expect(markup).toContain("No local data has been scanned or loaded");
    }
  });

  it("changes the active page through the navigation reducer", () => {
    expect(navigationReducer("overview", "environment")).toBe("environment");
    expect(navigationReducer("environment", "repositories")).toBe("repositories");
    expect(navigationReducer("repositories", "requirements")).toBe("requirements");
    expect(navigationReducer("requirements", "worklist")).toBe("worklist");
  });
});
