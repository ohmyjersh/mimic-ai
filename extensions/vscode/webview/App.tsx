import React, { useReducer, useCallback, useEffect } from "react";
import {
  StudioState,
  StudioAction,
  initialState,
  studioReducer,
  FragmentInfo,
  ResolveNode,
  ResolveEdge,
  SavedTemplate,
} from "./state";
import { usePostMessage, useMessageListener } from "./hooks/useVscodeApi";
import { useCompose } from "./hooks/useCompose";
import { PromptStudio } from "./components/PromptStudio";

interface RecommendResult {
  persona: string;
  skills: FragmentInfo[];
  contexts: FragmentInfo[];
  tones: FragmentInfo[];
  constraints: FragmentInfo[];
}

interface ExtensionMessage {
  type: string;
  fragments?: Record<string, FragmentInfo[]>;
  persona?: string;
  recommendations?: RecommendResult;
  markdown?: string;
  nodes?: ResolveNode[];
  edges?: ResolveEdge[];
  templates?: SavedTemplate[];
  message?: string;
  source?: string;
}

export function App() {
  const [state, dispatch] = useReducer(studioReducer, initialState);
  const postMessage = usePostMessage();

  // Listen for messages from extension host
  useMessageListener(
    useCallback((msg: unknown) => {
      const data = msg as ExtensionMessage;
      switch (data.type) {
        case "allFragmentsLoaded":
          if (data.fragments) {
            dispatch({ type: "SET_FRAGMENTS", fragments: data.fragments });
          }
          break;

        case "recommendationsLoaded":
          if (data.recommendations) {
            const rec = data.recommendations;
            dispatch({
              type: "SET_RECOMMENDATIONS",
              skills: rec.skills.map((f) => f.name),
              contexts: rec.contexts.map((f) => f.name),
              tones: rec.tones.map((f) => f.name),
              constraints: rec.constraints.map((f) => f.name),
            });
          }
          break;

        case "composeResult":
          dispatch({ type: "SET_COMPOSED", markdown: data.markdown ?? "" });
          break;

        case "resolveResult":
          dispatch({
            type: "SET_GRAPH",
            nodes: data.nodes ?? [],
            edges: data.edges ?? [],
          });
          break;

        case "templatesLoaded":
          dispatch({ type: "SET_TEMPLATES", templates: data.templates ?? [] });
          break;

        case "error":
          console.error(`mimic error [${data.source}]: ${data.message}`);
          break;
      }
    }, [])
  );

  // Signal ready on mount
  useEffect(() => {
    postMessage({ type: "ready" });
    postMessage({ type: "loadTemplates" });
  }, [postMessage]);

  // Fetch recommendations when persona changes
  useEffect(() => {
    if (state.selectedPersona) {
      postMessage({ type: "fetchRecommendations", persona: state.selectedPersona });
    }
  }, [state.selectedPersona, postMessage]);

  // Fetch graph data when a persona is drilled into on the graph tab
  useEffect(() => {
    if (state.activeTab === "graph" && state.graphPersona) {
      postMessage({
        type: "resolve",
        persona: state.graphPersona,
        tags: [],
        groups: [],
      });
    }
  }, [state.activeTab, state.graphPersona, postMessage]);

  // Debounced compose on selection change
  const onComposing = useCallback(() => {
    dispatch({ type: "SET_COMPOSING", value: true });
  }, []);

  useCompose(
    state.selectedPersona,
    state.selectedSkills,
    state.selectedContexts,
    state.selectedTones,
    state.selectedConstraints,
    onComposing
  );

  return <PromptStudio state={state} dispatch={dispatch} />;
}
