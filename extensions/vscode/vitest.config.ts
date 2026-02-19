import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    environment: "jsdom",
    globals: true,
    include: ["webview/**/*.test.{ts,tsx}", "src/**/*.test.ts"],
  },
});
