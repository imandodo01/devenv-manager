import { describe, expect, it } from "vitest";
import App from "./App";

describe("application foundation", () => {
  it("renders the foundation screen root", () => {
    const element = App();

    expect(element.type).toBe("main");
    expect(element.props["data-testid"]).toBe("app-root");
  });
});
