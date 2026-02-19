import React from "react";
import { useSlotDrop } from "../hooks/useDragDrop";
import { BuildItem } from "./BuildItem";

const CATEGORY_COLORS: Record<string, string> = {
  persona: "var(--cat-persona, var(--studio-muted))",
  skill: "var(--cat-skill)",
  context: "var(--cat-context)",
  tone: "var(--cat-tone)",
  constraint: "var(--cat-constraint)",
};

interface BuildSlotProps {
  title: string;
  category: string;
  items: string[];
  onAdd: (name: string) => void;
  onRemove: (name: string) => void;
  onReorder: (names: string[]) => void;
  dragCategory: string | null;
}

export function BuildSlot({
  title,
  category,
  items,
  onAdd,
  onRemove,
  onReorder,
  dragCategory,
}: BuildSlotProps) {
  const { onDragOver, onDragLeave, onDrop, isOver } = useSlotDrop(
    category,
    onAdd,
    onReorder,
    items
  );

  const isEmpty = items.length === 0;
  const isDropTarget = dragCategory === category;

  const classes = [
    "build-slot",
    isOver ? "drag-over" : "",
    isEmpty ? "empty" : "",
    isDropTarget && !isOver ? "drop-target" : "",
  ]
    .filter(Boolean)
    .join(" ");

  return (
    <div
      className={classes}
      onDragOver={onDragOver}
      onDragLeave={onDragLeave}
      onDrop={onDrop}
    >
      <div className="build-slot-header">
        <span
          className="build-slot-indicator"
          style={{ background: CATEGORY_COLORS[category] ?? "var(--studio-muted)" }}
        />
        <span className="build-slot-title">{title}</span>
        {isDropTarget && <span className="build-slot-drop-badge">+</span>}
      </div>
      <div className="build-slot-items">
        {items.map((name, i) => (
          <BuildItem
            key={name}
            name={name}
            category={category}
            index={i}
            onRemove={() => onRemove(name)}
          />
        ))}
        {isEmpty && !isDropTarget && (
          <div className="build-slot-empty">
            Drag {title.toLowerCase()} here
          </div>
        )}
        {isEmpty && isDropTarget && (
          <div className="build-slot-empty drop-hint">
            Drop here
          </div>
        )}
      </div>
    </div>
  );
}
