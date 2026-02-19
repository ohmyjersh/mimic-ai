import * as vscode from "vscode";
import { SavedTemplate } from "../studio/message-protocol";

const STORAGE_KEY = "mimic.templates";

export class TemplateStore {
  constructor(private readonly context: vscode.ExtensionContext) {}

  list(): SavedTemplate[] {
    return this.context.globalState.get<SavedTemplate[]>(STORAGE_KEY, []);
  }

  save(template: SavedTemplate): void {
    const templates = this.list().filter((t) => t.name !== template.name);
    templates.push(template);
    this.context.globalState.update(STORAGE_KEY, templates);
  }

  delete(name: string): void {
    const templates = this.list().filter((t) => t.name !== name);
    this.context.globalState.update(STORAGE_KEY, templates);
  }

  get(name: string): SavedTemplate | undefined {
    return this.list().find((t) => t.name === name);
  }
}
