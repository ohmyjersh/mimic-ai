import { FragmentInfo, RecommendResult } from "../picker";

// --- Webview → Extension ---

export interface ReadyMessage {
  type: "ready";
}

export interface FetchFragmentsMessage {
  type: "fetchFragments";
}

export interface FetchRecommendationsMessage {
  type: "fetchRecommendations";
  persona: string;
}

export interface ComposeMessage {
  type: "compose";
  persona: string;
  skills: string[];
  contexts: string[];
  tones: string[];
  constraints: string[];
}

export interface ResolveMessage {
  type: "resolve";
  persona?: string;
  tags: string[];
  groups: string[];
}

export interface InsertIntoEditorMessage {
  type: "insertIntoEditor";
  text: string;
}

export interface CopyToClipboardMessage {
  type: "copyToClipboard";
  text: string;
}

export interface SaveTemplateMessage {
  type: "saveTemplate";
  name: string;
  selections: TemplateSelections;
}

export interface LoadTemplatesMessage {
  type: "loadTemplates";
}

export type WebviewMessage =
  | ReadyMessage
  | FetchFragmentsMessage
  | FetchRecommendationsMessage
  | ComposeMessage
  | ResolveMessage
  | InsertIntoEditorMessage
  | CopyToClipboardMessage
  | SaveTemplateMessage
  | LoadTemplatesMessage;

// --- Extension → Webview ---

export interface AllFragmentsLoadedMessage {
  type: "allFragmentsLoaded";
  fragments: Record<string, FragmentInfo[]>;
}

export interface RecommendationsLoadedMessage {
  type: "recommendationsLoaded";
  persona: string;
  recommendations: RecommendResult;
}

export interface ComposeResultMessage {
  type: "composeResult";
  markdown: string;
}

export interface ResolveResultMessage {
  type: "resolveResult";
  nodes: ResolveNode[];
  edges: ResolveEdge[];
}

export interface TemplatesLoadedMessage {
  type: "templatesLoaded";
  templates: SavedTemplate[];
}

export interface ErrorMessage {
  type: "error";
  message: string;
  source: string;
}

export type ExtensionMessage =
  | AllFragmentsLoadedMessage
  | RecommendationsLoadedMessage
  | ComposeResultMessage
  | ResolveResultMessage
  | TemplatesLoadedMessage
  | ErrorMessage;

// --- Shared types ---

export interface ResolveNode {
  name: string;
  category: string;
  description: string;
}

export interface ResolveEdge {
  from: string;
  to: string;
  kind: string;
}

export interface TemplateSelections {
  persona: string | null;
  skills: string[];
  contexts: string[];
  tones: string[];
  constraints: string[];
}

export interface SavedTemplate {
  name: string;
  selections: TemplateSelections;
  createdAt: string;
}
