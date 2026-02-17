import * as vscode from "vscode";
import { MimicClient } from "./mimic-client";
import {
  fetchFragments,
  parseRecommendResult,
  extractToolText,
  FragmentInfo,
} from "./picker";

/**
 * Show a single-select QuickPick for a given category.
 * Returns the selected fragment name, or undefined if the user cancels entirely.
 * The user can press the "Skip" button to skip this step (returns null).
 */
async function pickOne(
  fragments: FragmentInfo[],
  title: string,
  placeholder: string,
  required: boolean
): Promise<string | null | undefined> {
  const items: vscode.QuickPickItem[] = fragments.map((f) => ({
    label: f.name,
    detail: f.description,
  }));

  if (!required) {
    items.unshift({
      label: "$(dash) Skip",
      detail: "Skip this step",
    });
  }

  const picked = await vscode.window.showQuickPick(items, {
    title,
    placeHolder: placeholder,
  });

  if (!picked) {
    return undefined; // user cancelled
  }
  if (picked.label === "$(dash) Skip") {
    return null; // user chose to skip
  }
  return picked.label;
}

/**
 * Show a multi-select QuickPick for a given category.
 * Returns selected fragment names, or undefined if the user cancels entirely.
 * Empty array means the user confirmed with nothing selected (skip).
 */
async function pickMany(
  fragments: FragmentInfo[],
  title: string,
  placeholder: string
): Promise<string[] | undefined> {
  const items: vscode.QuickPickItem[] = fragments.map((f) => ({
    label: f.name,
    detail: f.description,
    picked: false,
  }));

  const picked = await vscode.window.showQuickPick(items, {
    title,
    placeHolder: placeholder,
    canPickMany: true,
  });

  if (!picked) {
    return undefined; // user cancelled
  }
  return picked.map((p) => p.label);
}

/**
 * Run the multi-step compose flow:
 * 1. Pick persona (required)
 * 2. Call recommend for that persona to get relevant fragments
 * 3. Pick skills (multi-select, optional)
 * 4. Pick context (optional)
 * 5. Pick tone (optional)
 * 6. Pick constraints (multi-select, optional)
 *
 * Then call the compose tool and insert the result.
 */
export async function runCompose(client: MimicClient): Promise<void> {
  try {
    // Fetch personas
    const personas = await fetchFragments(client, "persona");

    if (personas.length === 0) {
      vscode.window.showErrorMessage(
        "No personas found. Add persona fragments to get started."
      );
      return;
    }

    // Step 1: Pick persona (required)
    const persona = await pickOne(
      personas,
      "Step 1/5: Choose a Persona",
      "Select a persona (required)",
      true
    );
    if (persona === undefined || persona === null) {
      return;
    }

    // Fetch recommended fragments for this persona
    const recommendResult = await client.callTool("recommend", {
      persona,
    });
    const recommended = parseRecommendResult(recommendResult);

    // Fall back to fetching all fragments if recommend fails
    const skills = recommended?.skills ?? await fetchFragments(client, "skill");
    const contexts = recommended?.contexts ?? await fetchFragments(client, "context");
    const tones = recommended?.tones ?? await fetchFragments(client, "tone");
    const constraints = recommended?.constraints ?? await fetchFragments(client, "constraint");

    // Step 2: Pick skills (multi-select, optional)
    let selectedSkills: string[] = [];
    if (skills.length > 0) {
      const result = await pickMany(
        skills,
        "Step 2/5: Choose Skills",
        "Select skills (optional — confirm empty to skip)"
      );
      if (result === undefined) {
        return;
      }
      selectedSkills = result;
    }

    // Step 3: Pick context (optional)
    let selectedContext: string | null = null;
    if (contexts.length > 0) {
      const result = await pickOne(
        contexts,
        "Step 3/5: Choose Context",
        "Select a context (optional)",
        false
      );
      if (result === undefined) {
        return;
      }
      selectedContext = result;
    }

    // Step 4: Pick tone (optional)
    let selectedTone: string | null = null;
    if (tones.length > 0) {
      const result = await pickOne(
        tones,
        "Step 4/5: Choose Tone",
        "Select a tone (optional)",
        false
      );
      if (result === undefined) {
        return;
      }
      selectedTone = result;
    }

    // Step 5: Pick constraints (multi-select, optional)
    let selectedConstraints: string[] = [];
    if (constraints.length > 0) {
      const result = await pickMany(
        constraints,
        "Step 5/5: Choose Constraints",
        "Select constraints (optional — confirm empty to skip)"
      );
      if (result === undefined) {
        return;
      }
      selectedConstraints = result;
    }

    // Build the compose request
    const args: Record<string, unknown> = { persona };
    if (selectedSkills.length > 0) {
      args.skills = selectedSkills;
    }
    if (selectedContext) {
      args.context = selectedContext;
    }
    if (selectedTone) {
      args.tone = selectedTone;
    }
    if (selectedConstraints.length > 0) {
      args.constraints = selectedConstraints;
    }

    // Call the compose tool
    const result = await client.callTool("compose", args);
    const text = extractToolText(result);

    if (!text) {
      vscode.window.showErrorMessage("Compose returned no output.");
      return;
    }

    // Insert into active editor or open a new untitled document
    const editor = vscode.window.activeTextEditor;
    if (editor) {
      await editor.edit((editBuilder) => {
        editBuilder.insert(editor.selection.active, text);
      });
    } else {
      const doc = await vscode.workspace.openTextDocument({
        content: text,
        language: "markdown",
      });
      await vscode.window.showTextDocument(doc);
    }
  } catch (err: unknown) {
    const message = err instanceof Error ? err.message : String(err);
    vscode.window.showErrorMessage(`Mimic compose failed: ${message}`);
  }
}
