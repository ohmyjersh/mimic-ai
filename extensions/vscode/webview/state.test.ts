import { describe, it, expect } from "vitest";
import { studioReducer, initialState, StudioState } from "./state";

function stateWith(overrides: Partial<StudioState>): StudioState {
  return { ...initialState, ...overrides };
}

describe("studioReducer", () => {
  describe("ADD_FRAGMENT", () => {
    it("appends skill to selectedSkills", () => {
      const state = stateWith({ selectedSkills: ["a"] });
      const result = studioReducer(state, {
        type: "ADD_FRAGMENT",
        category: "skill",
        name: "b",
      });
      expect(result.selectedSkills).toEqual(["a", "b"]);
    });

    it("appends context to selectedContexts", () => {
      const result = studioReducer(initialState, {
        type: "ADD_FRAGMENT",
        category: "context",
        name: "ctx-1",
      });
      expect(result.selectedContexts).toEqual(["ctx-1"]);
    });

    it("appends tone to selectedTones", () => {
      const result = studioReducer(initialState, {
        type: "ADD_FRAGMENT",
        category: "tone",
        name: "professional",
      });
      expect(result.selectedTones).toEqual(["professional"]);
    });

    it("appends constraint to selectedConstraints", () => {
      const result = studioReducer(initialState, {
        type: "ADD_FRAGMENT",
        category: "constraint",
        name: "no-jargon",
      });
      expect(result.selectedConstraints).toEqual(["no-jargon"]);
    });

    it("does not duplicate if already selected", () => {
      const state = stateWith({ selectedSkills: ["a"] });
      const result = studioReducer(state, {
        type: "ADD_FRAGMENT",
        category: "skill",
        name: "a",
      });
      expect(result.selectedSkills).toEqual(["a"]);
    });

    it("preserves other state fields", () => {
      const state = stateWith({
        selectedPersona: "dev",
        selectedTones: ["formal"],
      });
      const result = studioReducer(state, {
        type: "ADD_FRAGMENT",
        category: "skill",
        name: "testing",
      });
      expect(result.selectedPersona).toBe("dev");
      expect(result.selectedTones).toEqual(["formal"]);
    });
  });

  describe("REMOVE_FRAGMENT", () => {
    it("removes skill from selectedSkills", () => {
      const state = stateWith({ selectedSkills: ["a", "b", "c"] });
      const result = studioReducer(state, {
        type: "REMOVE_FRAGMENT",
        category: "skill",
        name: "b",
      });
      expect(result.selectedSkills).toEqual(["a", "c"]);
    });

    it("removes context from selectedContexts", () => {
      const state = stateWith({ selectedContexts: ["ctx-1", "ctx-2"] });
      const result = studioReducer(state, {
        type: "REMOVE_FRAGMENT",
        category: "context",
        name: "ctx-1",
      });
      expect(result.selectedContexts).toEqual(["ctx-2"]);
    });

    it("is a no-op when name not present", () => {
      const state = stateWith({ selectedSkills: ["a"] });
      const result = studioReducer(state, {
        type: "REMOVE_FRAGMENT",
        category: "skill",
        name: "nonexistent",
      });
      expect(result.selectedSkills).toEqual(["a"]);
    });

    it("results in empty array when removing last item", () => {
      const state = stateWith({ selectedTones: ["only-one"] });
      const result = studioReducer(state, {
        type: "REMOVE_FRAGMENT",
        category: "tone",
        name: "only-one",
      });
      expect(result.selectedTones).toEqual([]);
    });
  });

  describe("REORDER_FRAGMENTS", () => {
    it("replaces skill order entirely", () => {
      const state = stateWith({ selectedSkills: ["a", "b", "c"] });
      const result = studioReducer(state, {
        type: "REORDER_FRAGMENTS",
        category: "skill",
        names: ["c", "a", "b"],
      });
      expect(result.selectedSkills).toEqual(["c", "a", "b"]);
    });

    it("replaces constraint order", () => {
      const state = stateWith({ selectedConstraints: ["x", "y"] });
      const result = studioReducer(state, {
        type: "REORDER_FRAGMENTS",
        category: "constraint",
        names: ["y", "x"],
      });
      expect(result.selectedConstraints).toEqual(["y", "x"]);
    });

    it("can set to empty array", () => {
      const state = stateWith({ selectedContexts: ["a", "b"] });
      const result = studioReducer(state, {
        type: "REORDER_FRAGMENTS",
        category: "context",
        names: [],
      });
      expect(result.selectedContexts).toEqual([]);
    });
  });

  describe("existing actions still work", () => {
    it("SELECT_PERSONA sets persona and clears recommendations", () => {
      const state = stateWith({
        recommendedSkills: ["a"],
        recommendedContexts: ["b"],
      });
      const result = studioReducer(state, {
        type: "SELECT_PERSONA",
        name: "senior-dev",
      });
      expect(result.selectedPersona).toBe("senior-dev");
      expect(result.recommendedSkills).toEqual([]);
      expect(result.recommendedContexts).toEqual([]);
    });

    it("TOGGLE_SKILL adds then removes", () => {
      let state = studioReducer(initialState, { type: "TOGGLE_SKILL", name: "x" });
      expect(state.selectedSkills).toEqual(["x"]);
      state = studioReducer(state, { type: "TOGGLE_SKILL", name: "x" });
      expect(state.selectedSkills).toEqual([]);
    });

    it("LOAD_TEMPLATE sets all selections", () => {
      const result = studioReducer(initialState, {
        type: "LOAD_TEMPLATE",
        selections: {
          persona: "p",
          skills: ["s1"],
          contexts: ["c1"],
          tones: ["t1"],
          constraints: ["con1"],
        },
      });
      expect(result.selectedPersona).toBe("p");
      expect(result.selectedSkills).toEqual(["s1"]);
      expect(result.selectedContexts).toEqual(["c1"]);
      expect(result.selectedTones).toEqual(["t1"]);
      expect(result.selectedConstraints).toEqual(["con1"]);
    });
  });
});
