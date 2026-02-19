import * as vscode from "vscode";
import { TemplateStore } from "../templates/template-store";
import { SavedTemplate } from "../studio/message-protocol";

class TemplateItem extends vscode.TreeItem {
  constructor(public readonly template: SavedTemplate) {
    super(template.name, vscode.TreeItemCollapsibleState.None);
    const parts: string[] = [];
    if (template.selections.persona) {
      parts.push(template.selections.persona);
    }
    const count =
      template.selections.skills.length +
      template.selections.contexts.length +
      template.selections.tones.length +
      template.selections.constraints.length;
    if (count > 0) {
      parts.push(`+${count} fragments`);
    }
    this.description = parts.join(" | ");
    this.tooltip = `Created: ${template.createdAt}`;
    this.contextValue = "template";
  }
}

export class TemplateTreeProvider
  implements vscode.TreeDataProvider<TemplateItem>
{
  private _onDidChangeTreeData = new vscode.EventEmitter<
    TemplateItem | undefined
  >();
  readonly onDidChangeTreeData = this._onDidChangeTreeData.event;

  constructor(private readonly store: TemplateStore) {}

  refresh(): void {
    this._onDidChangeTreeData.fire(undefined);
  }

  getTreeItem(element: TemplateItem): vscode.TreeItem {
    return element;
  }

  getChildren(): TemplateItem[] {
    return this.store
      .list()
      .map((t) => new TemplateItem(t));
  }
}
