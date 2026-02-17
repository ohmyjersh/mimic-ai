import * as vscode from "vscode";
import { MimicClient } from "./mimic-client";

export interface FragmentInfo {
  name: string;
  category: string;
  description: string;
  tags: string[];
}

export interface ToolResult {
  content?: Array<{ type: string; text?: string }>;
}

export interface RecommendResult {
  persona: string;
  skills: FragmentInfo[];
  contexts: FragmentInfo[];
  tones: FragmentInfo[];
  constraints: FragmentInfo[];
}

/**
 * Extract the text content from a tool result.
 */
export function extractToolText(result: unknown): string | undefined {
  const toolResult = result as ToolResult;
  return toolResult?.content?.find((c) => c.type === "text")?.text;
}

/**
 * Parse a recommend tool response into a RecommendResult.
 */
export function parseRecommendResult(result: unknown): RecommendResult | undefined {
  const text = extractToolText(result);
  if (!text) {
    return undefined;
  }
  try {
    return JSON.parse(text) as RecommendResult;
  } catch {
    return undefined;
  }
}

/**
 * Parse the list tool response into an array of FragmentInfo objects.
 */
function parseFragments(result: unknown): FragmentInfo[] {
  const toolResult = result as ToolResult;
  if (!toolResult?.content?.length) {
    return [];
  }

  const text = toolResult.content.find((c) => c.type === "text")?.text;
  if (!text) {
    return [];
  }

  try {
    return JSON.parse(text) as FragmentInfo[];
  } catch {
    return [];
  }
}

/**
 * Fetch all fragments from the mimic server, optionally filtered by category.
 */
export async function fetchFragments(
  client: MimicClient,
  category?: string
): Promise<FragmentInfo[]> {
  const args: Record<string, unknown> = {};
  if (category) {
    args.category = category;
  }
  const result = await client.callTool("list", args);
  return parseFragments(result);
}

/**
 * Show a QuickPick listing all available fragments, grouped by category.
 * Returns the selected fragment or undefined if cancelled.
 */
export async function showFragmentList(
  client: MimicClient
): Promise<FragmentInfo | undefined> {
  const fragments = await fetchFragments(client);

  if (fragments.length === 0) {
    vscode.window.showInformationMessage("No fragments found.");
    return undefined;
  }

  const items: vscode.QuickPickItem[] = fragments.map((f) => ({
    label: f.name,
    description: f.category,
    detail: f.description + (f.tags.length ? ` [${f.tags.join(", ")}]` : ""),
  }));

  // Sort by category then name for grouping
  items.sort((a, b) => {
    const catCmp = (a.description ?? "").localeCompare(b.description ?? "");
    if (catCmp !== 0) {
      return catCmp;
    }
    return a.label.localeCompare(b.label);
  });

  const picked = await vscode.window.showQuickPick(items, {
    title: "Mimic Fragments",
    placeHolder: "Select a fragment to view details",
    matchOnDescription: true,
    matchOnDetail: true,
  });

  if (!picked) {
    return undefined;
  }

  return fragments.find(
    (f) => f.name === picked.label && f.category === picked.description
  );
}
