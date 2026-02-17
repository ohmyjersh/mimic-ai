import * as vscode from "vscode";
import { MimicClient } from "./mimic-client";
import { fetchFragments, parseRecommendResult, FragmentInfo } from "./picker";

/**
 * Run the recommend command: pick a persona, then show categorized recommendations.
 */
export async function runRecommend(client: MimicClient): Promise<void> {
  const personas = await fetchFragments(client, "persona");
  if (personas.length === 0) {
    vscode.window.showErrorMessage(
      "No personas found. Add persona fragments to get started."
    );
    return;
  }

  const personaPick = await vscode.window.showQuickPick(
    personas.map((p) => ({ label: p.name, detail: p.description })),
    { title: "Choose a Persona", placeHolder: "Select a persona to get recommendations" }
  );
  if (!personaPick) {
    return;
  }

  const result = await client.callTool("recommend", {
    persona: personaPick.label,
  });
  const recommend = parseRecommendResult(result);
  if (!recommend) {
    vscode.window.showErrorMessage("Recommend returned no results.");
    return;
  }

  const items: vscode.QuickPickItem[] = [];

  const addCategory = (label: string, fragments: FragmentInfo[]) => {
    if (fragments.length === 0) {
      return;
    }
    items.push({ label, kind: vscode.QuickPickItemKind.Separator });
    for (const f of fragments) {
      items.push({ label: f.name, description: f.category, detail: f.description });
    }
  };

  addCategory("Skills", recommend.skills);
  addCategory("Contexts", recommend.contexts);
  addCategory("Tones", recommend.tones);
  addCategory("Constraints", recommend.constraints);

  if (items.length === 0) {
    vscode.window.showInformationMessage(
      `No recommendations found for persona "${personaPick.label}".`
    );
    return;
  }

  await vscode.window.showQuickPick(items, {
    title: `Recommendations for "${personaPick.label}"`,
    placeHolder: "Browse recommended fragments",
    matchOnDescription: true,
    matchOnDetail: true,
  });
}
