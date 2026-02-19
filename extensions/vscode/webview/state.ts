export interface FragmentInfo {
  name: string;
  category: string;
  description: string;
  tags: string[];
  group?: string;
  level?: string;
  source?: string;
}

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

export interface SavedTemplate {
  name: string;
  selections: TemplateSelections;
  createdAt: string;
}

export interface TemplateSelections {
  persona: string | null;
  skills: string[];
  contexts: string[];
  tones: string[];
  constraints: string[];
}

export interface StudioState {
  // Fragment data
  personas: FragmentInfo[];
  skills: FragmentInfo[];
  contexts: FragmentInfo[];
  tones: FragmentInfo[];
  constraints: FragmentInfo[];

  // Selections
  selectedPersona: string | null;
  selectedSkills: string[];
  selectedContexts: string[];
  selectedTones: string[];
  selectedConstraints: string[];

  // Recommendations
  recommendedSkills: string[];
  recommendedContexts: string[];
  recommendedTones: string[];
  recommendedConstraints: string[];

  // Preview
  composedMarkdown: string;
  isComposing: boolean;

  // Graph
  graphPersona: string | null; // persona drilled into, null = overview
  graphNodes: ResolveNode[];
  graphEdges: ResolveEdge[];

  // UI
  searchQuery: string;
  activeTab: "builder" | "graph";

  // Templates
  savedTemplates: SavedTemplate[];
}

export const initialState: StudioState = {
  personas: [],
  skills: [],
  contexts: [],
  tones: [],
  constraints: [],

  selectedPersona: null,
  selectedSkills: [],
  selectedContexts: [],
  selectedTones: [],
  selectedConstraints: [],

  recommendedSkills: [],
  recommendedContexts: [],
  recommendedTones: [],
  recommendedConstraints: [],

  composedMarkdown: "",
  isComposing: false,

  graphPersona: null,
  graphNodes: [],
  graphEdges: [],

  searchQuery: "",
  activeTab: "builder",

  savedTemplates: [],
};

export type StudioAction =
  | { type: "SET_FRAGMENTS"; fragments: Record<string, FragmentInfo[]> }
  | { type: "SELECT_PERSONA"; name: string | null }
  | { type: "TOGGLE_SKILL"; name: string }
  | { type: "TOGGLE_CONTEXT"; name: string }
  | { type: "TOGGLE_TONE"; name: string }
  | { type: "TOGGLE_CONSTRAINT"; name: string }
  | {
      type: "SET_RECOMMENDATIONS";
      skills: string[];
      contexts: string[];
      tones: string[];
      constraints: string[];
    }
  | { type: "SET_COMPOSED"; markdown: string }
  | { type: "SET_COMPOSING"; value: boolean }
  | { type: "SET_GRAPH"; nodes: ResolveNode[]; edges: ResolveEdge[] }
  | { type: "SET_GRAPH_PERSONA"; persona: string | null }
  | { type: "SET_SEARCH"; query: string }
  | { type: "SET_TAB"; tab: "builder" | "graph" }
  | { type: "SET_TEMPLATES"; templates: SavedTemplate[] }
  | { type: "LOAD_TEMPLATE"; selections: TemplateSelections }
  | { type: "ADD_FRAGMENT"; category: "skill" | "context" | "tone" | "constraint"; name: string }
  | { type: "REMOVE_FRAGMENT"; category: "skill" | "context" | "tone" | "constraint"; name: string }
  | { type: "REORDER_FRAGMENTS"; category: "skill" | "context" | "tone" | "constraint"; names: string[] };

function toggle(arr: string[], name: string): string[] {
  return arr.includes(name) ? arr.filter((n) => n !== name) : [...arr, name];
}

export function studioReducer(
  state: StudioState,
  action: StudioAction
): StudioState {
  switch (action.type) {
    case "SET_FRAGMENTS":
      return {
        ...state,
        personas: action.fragments["persona"] ?? [],
        skills: action.fragments["skill"] ?? [],
        contexts: action.fragments["context"] ?? [],
        tones: action.fragments["tone"] ?? [],
        constraints: action.fragments["constraint"] ?? [],
      };

    case "SELECT_PERSONA":
      return {
        ...state,
        selectedPersona: action.name,
        // Clear recommendations when persona changes
        recommendedSkills: [],
        recommendedContexts: [],
        recommendedTones: [],
        recommendedConstraints: [],
      };

    case "TOGGLE_SKILL":
      return { ...state, selectedSkills: toggle(state.selectedSkills, action.name) };
    case "TOGGLE_CONTEXT":
      return { ...state, selectedContexts: toggle(state.selectedContexts, action.name) };
    case "TOGGLE_TONE":
      return { ...state, selectedTones: toggle(state.selectedTones, action.name) };
    case "TOGGLE_CONSTRAINT":
      return { ...state, selectedConstraints: toggle(state.selectedConstraints, action.name) };

    case "SET_RECOMMENDATIONS":
      return {
        ...state,
        recommendedSkills: action.skills,
        recommendedContexts: action.contexts,
        recommendedTones: action.tones,
        recommendedConstraints: action.constraints,
      };

    case "SET_COMPOSED":
      return { ...state, composedMarkdown: action.markdown, isComposing: false };
    case "SET_COMPOSING":
      return { ...state, isComposing: action.value };

    case "SET_GRAPH":
      return { ...state, graphNodes: action.nodes, graphEdges: action.edges };
    case "SET_GRAPH_PERSONA":
      return {
        ...state,
        graphPersona: action.persona,
        // Clear graph data when changing persona so old data doesn't flash
        graphNodes: [],
        graphEdges: [],
      };

    case "SET_SEARCH":
      return { ...state, searchQuery: action.query };
    case "SET_TAB":
      return { ...state, activeTab: action.tab };
    case "SET_TEMPLATES":
      return { ...state, savedTemplates: action.templates };

    case "LOAD_TEMPLATE":
      return {
        ...state,
        selectedPersona: action.selections.persona,
        selectedSkills: action.selections.skills,
        selectedContexts: action.selections.contexts,
        selectedTones: action.selections.tones,
        selectedConstraints: action.selections.constraints,
      };

    case "ADD_FRAGMENT":
      return addFragment(state, action.category, action.name);
    case "REMOVE_FRAGMENT":
      return removeFragment(state, action.category, action.name);
    case "REORDER_FRAGMENTS":
      return reorderFragments(state, action.category, action.names);

    default:
      return state;
  }
}

type FragmentCategory = "skill" | "context" | "tone" | "constraint";

function selectedKey(category: FragmentCategory): keyof StudioState {
  const map: Record<FragmentCategory, keyof StudioState> = {
    skill: "selectedSkills",
    context: "selectedContexts",
    tone: "selectedTones",
    constraint: "selectedConstraints",
  };
  return map[category];
}

function addFragment(state: StudioState, category: FragmentCategory, name: string): StudioState {
  const key = selectedKey(category);
  const current = state[key] as string[];
  if (current.includes(name)) return state;
  return { ...state, [key]: [...current, name] };
}

function removeFragment(state: StudioState, category: FragmentCategory, name: string): StudioState {
  const key = selectedKey(category);
  const current = state[key] as string[];
  return { ...state, [key]: current.filter((n) => n !== name) };
}

function reorderFragments(state: StudioState, category: FragmentCategory, names: string[]): StudioState {
  const key = selectedKey(category);
  return { ...state, [key]: names };
}
