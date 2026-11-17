import { createElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it } from "vitest";
import App from "./App";
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

  it("changes the active page through the navigation reducer", () => {
    expect(navigationReducer("overview", "environment")).toBe("environment");
    expect(navigationReducer("environment", "repositories")).toBe("repositories");
    expect(navigationReducer("repositories", "requirements")).toBe("requirements");
    expect(navigationReducer("requirements", "worklist")).toBe("worklist");
  });
});
