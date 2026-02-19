import * as vscode from "vscode";
import { StudioProvider } from "./studio-provider";
import { MimicClient } from "../mimic-client";

export class StudioPanel {
  private static instance: StudioPanel | undefined;
  private readonly panel: vscode.WebviewPanel;
  private readonly provider: StudioProvider;
  private disposed = false;

  private constructor(
    panel: vscode.WebviewPanel,
    extensionUri: vscode.Uri,
    client: MimicClient,
    context: vscode.ExtensionContext
  ) {
    this.panel = panel;
    this.provider = new StudioProvider(panel.webview, client, context);

    this.panel.webview.html = this.getHtmlContent(extensionUri);

    this.panel.onDidDispose(() => {
      this.disposed = true;
      StudioPanel.instance = undefined;
    });
  }

  static open(
    extensionUri: vscode.Uri,
    client: MimicClient,
    context: vscode.ExtensionContext
  ): void {
    if (StudioPanel.instance) {
      StudioPanel.instance.panel.reveal();
      return;
    }

    const panel = vscode.window.createWebviewPanel(
      "mimicStudio",
      "Prompt Studio",
      vscode.ViewColumn.One,
      {
        enableScripts: true,
        retainContextWhenHidden: true,
        localResourceRoots: [vscode.Uri.joinPath(extensionUri, "out")],
      }
    );

    StudioPanel.instance = new StudioPanel(
      panel,
      extensionUri,
      client,
      context
    );
  }

  private getHtmlContent(extensionUri: vscode.Uri): string {
    const webview = this.panel.webview;
    const scriptUri = webview.asWebviewUri(
      vscode.Uri.joinPath(extensionUri, "out", "webview.js")
    );
    const nonce = getNonce();

    return `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src ${webview.cspSource} 'unsafe-inline'; script-src 'nonce-${nonce}'; font-src ${webview.cspSource};">
  <title>Prompt Studio</title>
</head>
<body>
  <div id="root"></div>
  <script nonce="${nonce}" src="${scriptUri}"></script>
</body>
</html>`;
  }
}

function getNonce(): string {
  const chars =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
  let result = "";
  for (let i = 0; i < 32; i++) {
    result += chars.charAt(Math.floor(Math.random() * chars.length));
  }
  return result;
}
