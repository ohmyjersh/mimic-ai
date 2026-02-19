import * as vscode from "vscode";
import { MimicClient } from "../mimic-client";
import {
  fetchFragments,
  extractToolText,
  parseRecommendResult,
  FragmentInfo,
} from "../picker";
import {
  WebviewMessage,
  ExtensionMessage,
  ResolveNode,
  ResolveEdge,
} from "./message-protocol";
import { TemplateStore } from "../templates/template-store";

export class StudioProvider {
  private readonly templateStore: TemplateStore;

  constructor(
    private readonly webview: vscode.Webview,
    private readonly client: MimicClient,
    context: vscode.ExtensionContext
  ) {
    this.templateStore = new TemplateStore(context);
    this.webview.onDidReceiveMessage((msg: WebviewMessage) =>
      this.handleMessage(msg)
    );
  }

  private async handleMessage(msg: WebviewMessage): Promise<void> {
    switch (msg.type) {
      case "ready":
      case "fetchFragments":
        return this.handleFetchFragments();
      case "fetchRecommendations":
        return this.handleFetchRecommendations(msg.persona);
      case "compose":
        return this.handleCompose(msg);
      case "resolve":
        return this.handleResolve(msg);
      case "insertIntoEditor":
        return this.handleInsert(msg.text);
      case "copyToClipboard":
        return this.handleCopy(msg.text);
      case "saveTemplate":
        return this.handleSaveTemplate(msg.name, msg.selections);
      case "loadTemplates":
        return this.handleLoadTemplates();
    }
  }

  private async handleFetchFragments(): Promise<void> {
    try {
      const all = await fetchFragments(this.client);
      const grouped: Record<string, FragmentInfo[]> = {};
      for (const f of all) {
        const cat = f.category.toLowerCase();
        if (!grouped[cat]) {
          grouped[cat] = [];
        }
        grouped[cat].push(f);
      }
      this.post({ type: "allFragmentsLoaded", fragments: grouped });
    } catch (err) {
      this.postError("fetchFragments", err);
    }
  }

  private async handleFetchRecommendations(persona: string): Promise<void> {
    try {
      const result = await this.client.callTool("recommend", { persona });
      const rec = parseRecommendResult(result);
      if (rec) {
        this.post({
          type: "recommendationsLoaded",
          persona,
          recommendations: rec,
        });
      } else {
        this.postError("recommend", new Error("Failed to parse recommendations"));
      }
    } catch (err) {
      this.postError("recommend", err);
    }
  }

  private async handleCompose(msg: {
    persona: string;
    skills: string[];
    contexts: string[];
    tones: string[];
    constraints: string[];
  }): Promise<void> {
    try {
      const args: Record<string, unknown> = { persona: msg.persona };
      if (msg.skills.length) args.skills = msg.skills;
      if (msg.contexts.length) args.contexts = msg.contexts;
      if (msg.tones.length) args.tones = msg.tones;
      if (msg.constraints.length) args.constraints = msg.constraints;

      const result = await this.client.callTool("compose", args);
      const text = extractToolText(result);
      this.post({ type: "composeResult", markdown: text ?? "" });
    } catch (err) {
      this.postError("compose", err);
    }
  }

  private async handleResolve(msg: {
    persona?: string;
    tags: string[];
    groups: string[];
  }): Promise<void> {
    try {
      const args: Record<string, unknown> = {};
      if (msg.persona) args.persona = msg.persona;
      if (msg.tags.length) args.tags = msg.tags;
      if (msg.groups.length) args.groups = msg.groups;

      const result = await this.client.callTool("resolve", args);
      const text = extractToolText(result);
      if (!text) {
        this.post({ type: "resolveResult", nodes: [], edges: [] });
        return;
      }

      const parsed = JSON.parse(text);
      const nodes: ResolveNode[] = (parsed.nodes ?? []).map(
        (f: { name: string; category: string; description?: string }) => ({
          name: f.name,
          category: f.category.toLowerCase(),
          description: f.description ?? "",
        })
      );
      const edges: ResolveEdge[] = (parsed.edges ?? []).map(
        (e: { from: string; to: string; relation?: string }) => ({
          from: e.from,
          to: e.to,
          kind: e.relation ?? "related",
        })
      );

      this.post({ type: "resolveResult", nodes, edges });
    } catch (err) {
      this.postError("resolve", err);
    }
  }

  private async handleInsert(text: string): Promise<void> {
    const editor = vscode.window.activeTextEditor;
    if (editor) {
      await editor.edit((edit) => {
        edit.insert(editor.selection.active, text);
      });
    } else {
      const doc = await vscode.workspace.openTextDocument({
        content: text,
        language: "markdown",
      });
      await vscode.window.showTextDocument(doc);
    }
  }

  private async handleCopy(text: string): Promise<void> {
    await vscode.env.clipboard.writeText(text);
    vscode.window.showInformationMessage("Prompt copied to clipboard.");
  }

  private async handleSaveTemplate(
    name: string,
    selections: {
      persona: string | null;
      skills: string[];
      contexts: string[];
      tones: string[];
      constraints: string[];
    }
  ): Promise<void> {
    this.templateStore.save({ name, selections, createdAt: new Date().toISOString() });
    await this.handleLoadTemplates();
  }

  private async handleLoadTemplates(): Promise<void> {
    const templates = this.templateStore.list();
    this.post({ type: "templatesLoaded", templates });
  }

  private post(msg: ExtensionMessage): void {
    this.webview.postMessage(msg);
  }

  private postError(source: string, err: unknown): void {
    const message = err instanceof Error ? err.message : String(err);
    this.post({ type: "error", message, source });
  }
}
