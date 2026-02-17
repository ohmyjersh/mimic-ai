import * as vscode from "vscode";
import { MimicClient } from "./mimic-client";
import { extractToolText } from "./picker";

/**
 * Run the check_update command: call the MCP tool and display version info.
 */
export async function runCheckUpdate(client: MimicClient): Promise<void> {
  const result = await client.callTool("check_update", {});
  const text = extractToolText(result);

  if (!text) {
    vscode.window.showErrorMessage("Check update returned no output.");
    return;
  }

  try {
    const info = JSON.parse(text) as {
      current_version?: string;
      latest_version?: string;
      update_available?: boolean;
    };

    if (info.update_available) {
      const action = await vscode.window.showInformationMessage(
        `Mimic update available: ${info.current_version} \u2192 ${info.latest_version}`,
        "Open Releases"
      );
      if (action === "Open Releases") {
        vscode.env.openExternal(
          vscode.Uri.parse("https://github.com/ohmyjersh/mimic-ai/releases/latest")
        );
      }
    } else {
      vscode.window.showInformationMessage(
        `Mimic is up to date (${info.current_version}).`
      );
    }
  } catch {
    // If the response isn't JSON, just show it as-is
    vscode.window.showInformationMessage(text);
  }
}
