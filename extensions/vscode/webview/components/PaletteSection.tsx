import React, { useState, useCallback, useRef } from "react";
import { FragmentInfo } from "../state";
import { filterFragments } from "../lib/filter";
import { PaletteItem } from "./PaletteItem";

const CATEGORY_COLORS: Record<string, string> = {
  persona: "var(--cat-persona, var(--studio-muted))",
  skill: "var(--cat-skill)",
  context: "var(--cat-context)",
  tone: "var(--cat-tone)",
  constraint: "var(--cat-constraint)",
};

interface PaletteSectionProps {
  title: string;
  category: string;
  fragments: FragmentInfo[];
  selected: string[];
  recommended: string[];
  isOpen: boolean;
  onToggle: () => void;
  onAdd: (name: string) => void;
  onRemove: (name: string) => void;
}

export function PaletteSection({
  title,
  category,
  fragments,
  selected,
  recommended,
  isOpen,
  onToggle,
  onAdd,
  onRemove,
}: PaletteSectionProps) {
  const [search, setSearch] = useState("");
  const timerRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  const handleSearchChange = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => {
      const val = e.target.value;
      if (timerRef.current) clearTimeout(timerRef.current);
      timerRef.current = setTimeout(() => setSearch(val), 150);
    },
    []
  );

  const filtered = filterFragments(fragments, search);
  const selectedSet = new Set(selected);
  const recommendedSet = new Set(recommended);

  return (
    <>
      <div className="palette-section-header" onClick={onToggle}>
        <span className={`palette-section-toggle${isOpen ? "" : " collapsed"}`}>
          &#9660;
        </span>
        <span
          className="palette-section-indicator"
          style={{ background: CATEGORY_COLORS[category] ?? "var(--studio-muted)" }}
        />
        <span className="palette-section-title">{title}</span>
        <span className="palette-section-count">{fragments.length}</span>
        {selected.length > 0 && (
          <span className="palette-section-selected">{selected.length} selected</span>
        )}
      </div>
      {isOpen && (
        <div className="palette-section-body">
          <div className="palette-search">
            <input
              className="palette-search-input"
              type="text"
              placeholder={`Search ${title.toLowerCase()}...`}
              defaultValue={search}
              onChange={handleSearchChange}
            />
          </div>
          <div className="palette-items">
            {filtered.map((f) => (
              <PaletteItem
                key={f.name}
                fragment={f}
                category={category}
                isSelected={selectedSet.has(f.name)}
                isRecommended={recommendedSet.has(f.name)}
                onAdd={() => onAdd(f.name)}
                onRemove={() => onRemove(f.name)}
              />
            ))}
            {filtered.length === 0 && (
              <div className="palette-empty">No matches</div>
            )}
          </div>
        </div>
      )}
    </>
  );
}
