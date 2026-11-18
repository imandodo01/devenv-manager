import { createElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it, vi } from "vitest";
import { EnvironmentContent } from "./EnvironmentPage";
import type { EnvironmentInventorySnapshot } from "../lib/environment";

const refresh = vi.fn();

const baseSnapshot: EnvironmentInventorySnapshot = {
  scannedAt: "1795000000",
  tools: [
    {
      tool: "node",
      label: "Node.js",
      status: "detected",
      installations: [
        {
          executablePath: "C:/Tools/node.exe",
          artifactPath: null,
          version: "22.23.4",
        },
      ],
      incomplete: false,
      notes: [],
    },
    {
      tool: "php",
      label: "PHP",
      status: "notFound",
      installations: [],
      incomplete: false,
      notes: [],
    },
    {
      tool: "java",
      label: "Java",
      status: "ambiguous",
      installations: [
        {
          executablePath: "C:/Java/jdk-21/bin/java.exe",
          artifactPath: null,
          version: "21.0.2",
        },
        {
          executablePath: "C:/Java/jdk-17/bin/java.exe",
          artifactPath: null,
          version: "17.0.12",
        },
      ],
      incomplete: false,
      notes: [],
    },
    {
      tool: "git",
      label: "Git",
      status: "inspectionError",
      installations: [],
      incomplete: true,
      notes: ["Version command exceeded the 3-second time limit."],
    },
  ],
};

describe("environment inventory presentation", () => {
  it("renders detected, missing, ambiguous, and inspection-error states", () => {
    const markup = renderToStaticMarkup(
      createElement(EnvironmentContent, {
        snapshot: baseSnapshot,
        loading: false,
        error: null,
        onRefresh: refresh,
      }),
    );

    expect(markup).toContain("22.23.4");
    expect(markup).toContain("Not found in approved locations");
    expect(markup).toContain("Multiple installations");
    expect(markup).toContain("Inspection incomplete");
    expect(markup).toContain("C:/Java/jdk-17/bin/java.exe");
    expect(markup).toContain("3-second time limit");
  });

  it("shows loading and command errors without inventing inventory data", () => {
    const loadingMarkup = renderToStaticMarkup(
      createElement(EnvironmentContent, {
        snapshot: null,
        loading: true,
        error: null,
        onRefresh: refresh,
      }),
    );
    const errorMarkup = renderToStaticMarkup(
      createElement(EnvironmentContent, {
        snapshot: null,
        loading: false,
        error: "Command unavailable",
        onRefresh: refresh,
      }),
    );

    expect(loadingMarkup).toContain("Inspecting approved locations");
    expect(errorMarkup).toContain("Command unavailable");
    expect(errorMarkup).not.toContain("Node.js");
  });
});
