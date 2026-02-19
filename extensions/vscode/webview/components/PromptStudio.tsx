import React, { useCallback, useState, useEffect } from "react";
import { StudioState, StudioAction, TemplateSelections } from "../state";
import { getCurrentDragCategory } from "../hooks/useDragDrop";
import { Palette } from "./Palette";
import { BuildZone } from "./BuildZone";
import { PreviewPanel } from "./PreviewPanel";
import { Toolbar } from "./Toolbar";
import { GraphView } from "./GraphView";

interface PromptStudioProps {
  state: StudioState;
  dispatch: React.Dispatch<StudioAction>;
}

export function PromptStudio({ state, dispatch }: PromptStudioProps) {
  const [dragCategory, setDragCategory] = useState<string | null>(null);

  useEffect(() => {
    const handleDragStart = () => {
      // Read from module-level drag state (set by usePaletteDrag)
      setDragCategory(getCurrentDragCategory());
    };
    const handleDragEnd = () => {
      setDragCategory(null);
    };
    document.addEventListener("dragstart", handleDragStart);
    document.addEventListener("dragend", handleDragEnd);
    return () => {
      document.removeEventListener("dragstart", handleDragStart);
      document.removeEventListener("dragend", handleDragEnd);
    };
  }, []);

  const handleGraphDrillDown = useCallback(
    (persona: string) => {
      dispatch({ type: "SET_GRAPH_PERSONA", persona });
    },
    [dispatch]
  );

  const handleGraphBack = useCallback(() => {
    dispatch({ type: "SET_GRAPH_PERSONA", persona: null });
  }, [dispatch]);

  const handleGraphToggle = useCallback(
    (name: string, category: string) => {
      switch (category) {
        case "persona":
          dispatch({
            type: "SELECT_PERSONA",
            name: name === state.selectedPersona ? null : name,
          });
          break;
        case "skill":
          dispatch({ type: "TOGGLE_SKILL", name });
          break;
        case "context":
          dispatch({ type: "TOGGLE_CONTEXT", name });
          break;
        case "tone":
          dispatch({ type: "TOGGLE_TONE", name });
          break;
        case "constraint":
          dispatch({ type: "TOGGLE_CONSTRAINT", name });
          break;
      }
    },
    [dispatch, state.selectedPersona]
  );

  const selectedSet = new Set([
    ...(state.selectedPersona ? [state.selectedPersona] : []),
    ...state.selectedSkills,
    ...state.selectedContexts,
    ...state.selectedTones,
    ...state.selectedConstraints,
  ]);

  const selections: TemplateSelections = {
    persona: state.selectedPersona,
    skills: state.selectedSkills,
    contexts: state.selectedContexts,
    tones: state.selectedTones,
    constraints: state.selectedConstraints,
  };

  return (
    <div className="studio">
      <div className="studio-header">
        <div className="tab-bar">
          <button
            className={`tab-btn${state.activeTab === "builder" ? " active" : ""}`}
            onClick={() => dispatch({ type: "SET_TAB", tab: "builder" })}
          >
            Builder
          </button>
          <button
            className={`tab-btn${state.activeTab === "graph" ? " active" : ""}`}
            onClick={() => dispatch({ type: "SET_TAB", tab: "graph" })}
          >
            Graph
          </button>
        </div>
        <Toolbar
          markdown={state.composedMarkdown}
          selections={selections}
          templates={state.savedTemplates}
          hasPersona={!!state.selectedPersona}
        />
      </div>

      <div className="studio-body">
        {state.activeTab === "builder" ? (
          <>
            <Palette state={state} dispatch={dispatch} />
            <BuildZone state={state} dispatch={dispatch} dragCategory={dragCategory} />
            <PreviewPanel
              markdown={state.composedMarkdown}
              isComposing={state.isComposing}
              hasPersona={!!state.selectedPersona}
            />
          </>
        ) : (
          <GraphView
            personas={state.personas}
            graphPersona={state.graphPersona}
            nodes={state.graphNodes}
            edges={state.graphEdges}
            selectedFragments={selectedSet}
            onDrillDown={handleGraphDrillDown}
            onBack={handleGraphBack}
            onToggle={handleGraphToggle}
          />
        )}
      </div>
    </div>
  );
}
