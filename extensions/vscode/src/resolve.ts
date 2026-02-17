import * as vscode from "vscode";
import { MimicClient } from "./mimic-client";
import { fetchFragments, extractToolText } from "./picker";

/**
 * Run the resolve command: optionally pick a persona and enter tags/groups,
 * then display the resolved fragment graph as JSON.
 */
export async function runResolve(client: MimicClient): Promise<void> {
  // Optional persona selection
  const personas = await fetchFragments(client, "persona");
  let persona: string | undefined;

  if (personas.length > 0) {
    const items: vscode.QuickPickItem[] = [
      { label: "$(dash) Skip", detail: "Resolve without a persona" },
      ...personas.map((p) => ({ label: p.name, detail: p.description })),
    ];

    const pick = await vscode.window.showQuickPick(items, {
      title: "Resolve: Choose a Persona (optional)",
      placeHolder: "Select a persona or skip",
    });
    if (!pick) {
      return;
    }
    if (pick.label !== "$(dash) Skip") {
      persona = pick.label;
    }
  }

  // Optional tags input
  const tagsInput = await vscode.window.showInputBox({
    title: "Resolve: Tags (optional)",
    placeHolder: "Enter comma-separated tags, or leave empty",
  });
  if (tagsInput === undefined) {
    return;
  }

  // Optional groups input
  const groupsInput = await vscode.window.showInputBox({
    title: "Resolve: Groups (optional)",
    placeHolder: "Enter comma-separated groups, or leave empty",
  });
  if (groupsInput === undefined) {
    return;
  }

  const args: Record<string, unknown> = {};
  if (persona) {
    args.persona = persona;
  }
  if (tagsInput.trim()) {
    args.tags = tagsInput.split(",").map((t) => t.trim()).filter(Boolean);
  }
  if (groupsInput.trim()) {
    args.groups = groupsInput.split(",").map((g) => g.trim()).filter(Boolean);
  }

  const result = await client.callTool("resolve", args);
  const text = extractToolText(result);

  if (!text) {
    vscode.window.showErrorMessage("Resolve returned no output.");
    return;
  }

  // Pretty-print and show in editor
  let formatted: string;
  try {
    formatted = JSON.stringify(JSON.parse(text), null, 2);
  } catch {
    formatted = text;
  }

  const doc = await vscode.workspace.openTextDocument({
    content: formatted,
    language: "json",
  });
  await vscode.window.showTextDocument(doc);
}
