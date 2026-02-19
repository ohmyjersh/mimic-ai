import React from "react";
import { createRoot } from "react-dom/client";
import { App } from "./App";

// Import styles as side effects — esbuild inlines them
import "./styles/theme.css";
import "./styles/studio.css";

const container = document.getElementById("root");
if (container) {
  const root = createRoot(container);
  root.render(<App />);
}
