import { mergeConfig, defineConfig } from "vitest/config";
import viteConfig from "./vite.config";

export default mergeConfig(
  viteConfig,
  defineConfig({
    test: {
      include: ["src/**/*.test.{ts,tsx}"],
      environment: "node",
      clearMocks: true,
    },
  }),
);
