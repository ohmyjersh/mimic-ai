import * as vscode from "vscode";
import { MimicClient } from "../mimic-client";
import { fetchFragments, FragmentInfo } from "../picker";

class FragmentItem extends vscode.TreeItem {
  constructor(
    public readonly label: string,
    public readonly collapsibleState: vscode.TreeItemCollapsibleState,
    public readonly category?: string,
    public readonly fragment?: FragmentInfo
  ) {
    super(label, collapsibleState);
    if (fragment) {
      this.description = fragment.description;
      this.tooltip = `${fragment.name}\n${fragment.description}${
        fragment.tags.length ? `\nTags: ${fragment.tags.join(", ")}` : ""
      }`;
    }
  }
}

const CATEGORY_ORDER = ["persona", "skill", "context", "tone", "constraint"];

export class FragmentTreeProvider
  implements vscode.TreeDataProvider<FragmentItem>
{
  private fragments: Record<string, FragmentInfo[]> = {};
  private _onDidChangeTreeData = new vscode.EventEmitter<
    FragmentItem | undefined
  >();
  readonly onDidChangeTreeData = this._onDidChangeTreeData.event;

  async refresh(client: MimicClient): Promise<void> {
    const all = await fetchFragments(client);
    this.fragments = {};
    for (const f of all) {
      const cat = f.category.toLowerCase();
      if (!this.fragments[cat]) {
        this.fragments[cat] = [];
      }
      this.fragments[cat].push(f);
    }
    this._onDidChangeTreeData.fire(undefined);
  }

  getTreeItem(element: FragmentItem): vscode.TreeItem {
    return element;
  }

  getChildren(element?: FragmentItem): FragmentItem[] {
    if (!element) {
      return CATEGORY_ORDER
        .filter((cat) => this.fragments[cat]?.length)
        .map(
          (cat) =>
            new FragmentItem(
              cat.charAt(0).toUpperCase() + cat.slice(1) + "s",
              vscode.TreeItemCollapsibleState.Collapsed,
              cat
            )
        );
    }

    if (element.category) {
      const frags = this.fragments[element.category] ?? [];
      return frags
        .sort((a, b) => a.name.localeCompare(b.name))
        .map(
          (f) =>
            new FragmentItem(
              f.name,
              vscode.TreeItemCollapsibleState.None,
              undefined,
              f
            )
        );
    }

    return [];
  }
}
