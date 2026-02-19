import { describe, it, expect } from "vitest";
import { filterFragments } from "./filter";
import { FragmentInfo } from "../state";

function makeFragment(overrides: Partial<FragmentInfo> = {}): FragmentInfo {
  return {
    name: "test-fragment",
    category: "skill",
    description: "A test fragment",
    tags: ["test"],
    ...overrides,
  };
}

describe("filterFragments", () => {
  const fragments: FragmentInfo[] = [
    makeFragment({ name: "code-review", description: "Review code", tags: ["quality"] }),
    makeFragment({ name: "testing", description: "Write tests", tags: ["quality", "tdd"] }),
    makeFragment({ name: "debugging", description: "Debug issues", tags: ["debug"] }),
  ];

  it("returns all fragments when query is empty", () => {
    expect(filterFragments(fragments, "")).toEqual(fragments);
  });

  it("filters by name", () => {
    const result = filterFragments(fragments, "code");
    expect(result).toHaveLength(1);
    expect(result[0].name).toBe("code-review");
  });

  it("filters by description", () => {
    const result = filterFragments(fragments, "debug");
    expect(result).toHaveLength(1);
    expect(result[0].name).toBe("debugging");
  });

  it("filters by tags", () => {
    const result = filterFragments(fragments, "tdd");
    expect(result).toHaveLength(1);
    expect(result[0].name).toBe("testing");
  });

  it("is case-insensitive", () => {
    const result = filterFragments(fragments, "CODE");
    expect(result).toHaveLength(1);
    expect(result[0].name).toBe("code-review");
  });

  it("matches multiple fragments", () => {
    const result = filterFragments(fragments, "quality");
    expect(result).toHaveLength(2);
  });

  it("returns empty array when no matches", () => {
    expect(filterFragments(fragments, "nonexistent")).toHaveLength(0);
  });

  it("handles empty fragments array", () => {
    expect(filterFragments([], "test")).toHaveLength(0);
  });
});
