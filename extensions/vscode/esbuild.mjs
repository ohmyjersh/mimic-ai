import * as esbuild from "esbuild";

const watch = process.argv.includes("--watch");

/** @type {esbuild.BuildOptions} */
const extensionConfig = {
  entryPoints: ["src/extension.ts"],
  bundle: true,
  outfile: "out/extension.js",
  external: ["vscode"],
  format: "cjs",
  platform: "node",
  target: "es2020",
  sourcemap: true,
};

/**
 * Plugin to inject CSS imports as <style> tags in the webview.
 * Webviews can't load external CSS files, so we inline them.
 */
const inlineCssPlugin = {
  name: "inline-css",
  setup(build) {
    build.onLoad({ filter: /\.css$/ }, async (args) => {
      const fs = await import("fs");
      const css = fs.readFileSync(args.path, "utf8");
      return {
        contents: `
          (function() {
            const style = document.createElement('style');
            style.textContent = ${JSON.stringify(css)};
            document.head.appendChild(style);
          })();
        `,
        loader: "js",
      };
    });
  },
};

/** @type {esbuild.BuildOptions} */
const webviewConfig = {
  entryPoints: ["webview/index.tsx"],
  bundle: true,
  outfile: "out/webview.js",
  format: "iife",
  platform: "browser",
  target: "es2020",
  sourcemap: true,
  jsx: "automatic",
  jsxImportSource: "react",
  plugins: [inlineCssPlugin],
  define: {
    "process.env.NODE_ENV": '"production"',
  },
};

async function build() {
  if (watch) {
    const [extCtx, webCtx] = await Promise.all([
      esbuild.context(extensionConfig),
      esbuild.context(webviewConfig),
    ]);
    await Promise.all([extCtx.watch(), webCtx.watch()]);
    console.log("Watching for changes...");
  } else {
    await Promise.all([
      esbuild.build(extensionConfig),
      esbuild.build(webviewConfig),
    ]);
    console.log("Build complete.");
  }
}

build().catch((err) => {
  console.error(err);
  process.exit(1);
});
