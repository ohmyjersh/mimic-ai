import * as vscode from "vscode";
import { MimicClient } from "./mimic-client";
import { showFragmentList } from "./picker";
import { runCompose } from "./compose";
import { runRecommend } from "./recommend";
import { runResolve } from "./resolve";
import { runCheckUpdate } from "./check-update";
import { ensureBinary } from "./download";

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

  const config = vscode.workspace.getConfiguration("mimic");
  const configuredPath = config.get<string>("binaryPath", "mimic");
  const autoDownload = config.get<boolean>("autoDownload", true);

  const binaryPath = await ensureBinary(
    configuredPath,
    globalStoragePath,
    autoDownload
  );

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

  context.subscriptions.push(
    registerCommand("mimic.compose", runCompose),
    registerCommand("mimic.list", (c) => showFragmentList(c).then(() => {})),
    registerCommand("mimic.recommend", runRecommend),
    registerCommand("mimic.resolve", runResolve),
    registerCommand("mimic.checkUpdate", runCheckUpdate)
  );
}

export function deactivate(): void {
  if (client) {
    client.dispose();
    client = undefined;
  }
}
