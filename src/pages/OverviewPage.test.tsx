import { createElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it } from "vitest";
import OverviewPage from "./OverviewPage";

describe("overview capability summary", () => {
  it("shows the M2 environment inventory as available and M3 as next", () => {
    const markup = renderToStaticMarkup(createElement(OverviewPage));

    expect(markup).toContain("Read-only inventory and runtime detection are available in M2.");
    const environmentCard = markup.match(/<article\\b[\\s\\S]*?<\\/article>/)?.[0];

    expect(markup).toContain("Available");
    expect(environmentCard).toContain("Ready to use");
    expect(environmentCard).not.toContain("Not available yet");
    expect(markup).not.toContain("Tool and runtime detection is planned for M2.");
    expect(markup).toContain("M3 · Runtime information");
  });
});
