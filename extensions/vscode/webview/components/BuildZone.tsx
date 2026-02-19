import React, { useMemo, useCallback } from "react";
import { StudioState, StudioAction } from "../state";
import { BuildSlot } from "./BuildSlot";
import { RecommendationBar } from "./RecommendationBar";

interface BuildZoneProps {
  state: StudioState;
  dispatch: React.Dispatch<StudioAction>;
  dragCategory: string | null;
}

type FragmentCategory = "skill" | "context" | "tone" | "constraint";

export function BuildZone({ state, dispatch, dragCategory }: BuildZoneProps) {
  const handleAdd = useCallback(
    (category: FragmentCategory, name: string) => {
      dispatch({ type: "ADD_FRAGMENT", category, name });
    },
    [dispatch]
  );

  const handleRemove = useCallback(
    (category: FragmentCategory, name: string) => {
      dispatch({ type: "REMOVE_FRAGMENT", category, name });
    },
    [dispatch]
  );

  const handleReorder = useCallback(
    (category: FragmentCategory, names: string[]) => {
      dispatch({ type: "REORDER_FRAGMENTS", category, names });
    },
    [dispatch]
  );

  const handlePersonaRemove = useCallback(
    () => {
      dispatch({ type: "SELECT_PERSONA", name: null });
    },
    [dispatch]
  );

  const handleRecommendationAdd = useCallback(
    (name: string, category: string) => {
      const cat = category as FragmentCategory;
      dispatch({ type: "ADD_FRAGMENT", category: cat, name });
    },
    [dispatch]
  );

  // Collect unselected recommendations
  const recommendations = useMemo(() => {
    const items: { name: string; category: string }[] = [];
    const addUnselected = (
      recommended: string[],
      selected: string[],
      category: string
    ) => {
      const selectedSet = new Set(selected);
      for (const name of recommended) {
        if (!selectedSet.has(name)) {
          items.push({ name, category });
        }
      }
    };

    addUnselected(state.recommendedSkills, state.selectedSkills, "skill");
    addUnselected(state.recommendedContexts, state.selectedContexts, "context");
    addUnselected(state.recommendedTones, state.selectedTones, "tone");
    addUnselected(state.recommendedConstraints, state.selectedConstraints, "constraint");

    return items;
  }, [
    state.recommendedSkills,
    state.recommendedContexts,
    state.recommendedTones,
    state.recommendedConstraints,
    state.selectedSkills,
    state.selectedContexts,
    state.selectedTones,
    state.selectedConstraints,
  ]);

  return (
    <div className="build-zone">
      <div className="build-zone-slots">
        {/* Persona slot — special: only one allowed, uses SELECT_PERSONA */}
        <BuildSlot
          title="Persona"
          category="persona"
          items={state.selectedPersona ? [state.selectedPersona] : []}
          onAdd={(name) =>
            dispatch({ type: "SELECT_PERSONA", name })
          }
          onRemove={handlePersonaRemove}
          onReorder={() => {}}
          dragCategory={dragCategory}
        />

        <BuildSlot
          title="Skills"
          category="skill"
          items={state.selectedSkills}
          onAdd={(name) => handleAdd("skill", name)}
          onRemove={(name) => handleRemove("skill", name)}
          onReorder={(names) => handleReorder("skill", names)}
          dragCategory={dragCategory}
        />

        <BuildSlot
          title="Contexts"
          category="context"
          items={state.selectedContexts}
          onAdd={(name) => handleAdd("context", name)}
          onRemove={(name) => handleRemove("context", name)}
          onReorder={(names) => handleReorder("context", names)}
          dragCategory={dragCategory}
        />

        <BuildSlot
          title="Tones"
          category="tone"
          items={state.selectedTones}
          onAdd={(name) => handleAdd("tone", name)}
          onRemove={(name) => handleRemove("tone", name)}
          onReorder={(names) => handleReorder("tone", names)}
          dragCategory={dragCategory}
        />

        <BuildSlot
          title="Constraints"
          category="constraint"
          items={state.selectedConstraints}
          onAdd={(name) => handleAdd("constraint", name)}
          onRemove={(name) => handleRemove("constraint", name)}
          onReorder={(names) => handleReorder("constraint", names)}
          dragCategory={dragCategory}
        />
      </div>

      <RecommendationBar
        recommendations={recommendations}
        onAdd={handleRecommendationAdd}
      />
    </div>
  );
}
