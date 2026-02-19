import * as vscode from "vscode";
import { MimicClient } from "./mimic-client";
import { showFragmentList } from "./picker";
import { runCompose } from "./compose";
import { runRecommend } from "./recommend";
import { runResolve } from "./resolve";
import { runCheckUpdate } from "./check-update";
import { ensureBinary } from "./download";
import { StudioPanel } from "./studio/studio-panel";
import { FragmentTreeProvider } from "./sidebar/fragment-tree-provider";
import { TemplateTreeProvider } from "./sidebar/template-tree-provider";
import { TemplateStore } from "./templates/template-store";

let client: MimicClient | undefined;
let globalStoragePath: string;

/**
 * Get or create the MimicClient singleton.
 * On first use, ensures the binary is available (auto-downloading if needed).
 */
async function getClient(): Promise<MimicClient> {
  if (client) {
    return client;
  }

  // MIMIC_DEV_BINARY env var overrides all resolution — set by the
  // "Run Extension (Dev Binary)" launch config so you can test against
  // a local cargo build without changing any settings.
  const devBinary = process.env.MIMIC_DEV_BINARY;
  let binaryPath: string;

  if (devBinary) {
    console.log(`mimic: using dev binary from MIMIC_DEV_BINARY: ${devBinary}`);
    binaryPath = devBinary;
  } else {
    const config = vscode.workspace.getConfiguration("mimic");
    const configuredPath = config.get<string>("binaryPath", "mimic");
    const autoDownload = config.get<boolean>("autoDownload", true);
    binaryPath = await ensureBinary(configuredPath, globalStoragePath, autoDownload);
  }

  client = new MimicClient(binaryPath);
  await client.initialize();
  return client;
}

/**
 * Wrap a command handler with standard error handling.
 */
function registerCommand(
  id: string,
  handler: (client: MimicClient) => Promise<void>
): vscode.Disposable {
  return vscode.commands.registerCommand(id, async () => {
    try {
      const c = await getClient();
      await handler(c);
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : String(err);
      vscode.window.showErrorMessage(`Mimic: ${message}`);
    }
  });
}

export function activate(context: vscode.ExtensionContext): void {
  console.log("mimic: extension activating");
  globalStoragePath = context.globalStorageUri.fsPath;

  const templateStore = new TemplateStore(context);
  const fragmentTree = new FragmentTreeProvider();
  const templateTree = new TemplateTreeProvider(templateStore);

  context.subscriptions.push(
    registerCommand("mimic.compose", runCompose),
    registerCommand("mimic.list", (c) => showFragmentList(c).then(() => {})),
    registerCommand("mimic.recommend", runRecommend),
    registerCommand("mimic.resolve", runResolve),
    registerCommand("mimic.checkUpdate", runCheckUpdate),

    vscode.commands.registerCommand("mimic.openStudio", async () => {
      try {
        const c = await getClient();
        StudioPanel.open(context.extensionUri, c, context);
      } catch (err: unknown) {
        const message = err instanceof Error ? err.message : String(err);
        vscode.window.showErrorMessage(`Mimic: ${message}`);
      }
    }),

    vscode.window.registerTreeDataProvider("mimicFragments", fragmentTree),
    vscode.window.registerTreeDataProvider("mimicTemplates", templateTree),

    vscode.commands.registerCommand("mimic.refreshFragments", async () => {
      try {
        const c = await getClient();
        await fragmentTree.refresh(c);
      } catch (err: unknown) {
        const message = err instanceof Error ? err.message : String(err);
        vscode.window.showErrorMessage(`Mimic: ${message}`);
      }
    }),

    vscode.commands.registerCommand(
      "mimic.deleteTemplate",
      (name: string) => {
        templateStore.delete(name);
        templateTree.refresh();
      }
    )
  );

  // Eagerly load fragments into sidebar when client becomes available
  getClient()
    .then((c) => fragmentTree.refresh(c))
    .catch(() => {});
}

export function deactivate(): void {
  if (client) {
    client.dispose();
    client = undefined;
  }
}
