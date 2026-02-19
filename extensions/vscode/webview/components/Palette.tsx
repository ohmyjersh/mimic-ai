import React, { useState, useCallback } from "react";
import { StudioState, StudioAction } from "../state";
import { PaletteSection } from "./PaletteSection";

interface PaletteProps {
  state: StudioState;
  dispatch: React.Dispatch<StudioAction>;
}

type SectionKey = "persona" | "skill" | "context" | "tone" | "constraint";

const SECTIONS: { key: SectionKey; title: string }[] = [
  { key: "persona", title: "Personas" },
  { key: "skill", title: "Skills" },
  { key: "context", title: "Contexts" },
  { key: "tone", title: "Tones" },
  { key: "constraint", title: "Constraints" },
];

export function Palette({ state, dispatch }: PaletteProps) {
  const [openSection, setOpenSection] = useState<SectionKey | null>("persona");

  const handleToggle = useCallback(
    (key: SectionKey) => {
      setOpenSection((prev) => (prev === key ? null : key));
    },
    []
  );

  const handleAdd = useCallback(
    (category: SectionKey, name: string) => {
      if (category === "persona") {
        dispatch({
          type: "SELECT_PERSONA",
          name: name === state.selectedPersona ? null : name,
        });
      } else {
        dispatch({ type: "ADD_FRAGMENT", category, name });
      }
    },
    [dispatch, state.selectedPersona]
  );

  const handleRemove = useCallback(
    (category: SectionKey, name: string) => {
      if (category === "persona") {
        dispatch({ type: "SELECT_PERSONA", name: null });
      } else {
        dispatch({ type: "REMOVE_FRAGMENT", category, name });
      }
    },
    [dispatch]
  );

  const getFragments = (key: SectionKey) => {
    switch (key) {
      case "persona": return state.personas;
      case "skill": return state.skills;
      case "context": return state.contexts;
      case "tone": return state.tones;
      case "constraint": return state.constraints;
    }
  };

  const getSelected = (key: SectionKey): string[] => {
    switch (key) {
      case "persona": return state.selectedPersona ? [state.selectedPersona] : [];
      case "skill": return state.selectedSkills;
      case "context": return state.selectedContexts;
      case "tone": return state.selectedTones;
      case "constraint": return state.selectedConstraints;
    }
  };

  const getRecommended = (key: SectionKey): string[] => {
    switch (key) {
      case "persona": return [];
      case "skill": return state.recommendedSkills;
      case "context": return state.recommendedContexts;
      case "tone": return state.recommendedTones;
      case "constraint": return state.recommendedConstraints;
    }
  };

  return (
    <div className="palette">
      {SECTIONS.map(({ key, title }) => (
        <PaletteSection
          key={key}
          title={title}
          category={key}
          fragments={getFragments(key)}
          selected={getSelected(key)}
          recommended={getRecommended(key)}
          isOpen={openSection === key}
          onToggle={() => handleToggle(key)}
          onAdd={(name) => handleAdd(key, name)}
          onRemove={(name) => handleRemove(key, name)}
        />
      ))}
    </div>
  );
}
