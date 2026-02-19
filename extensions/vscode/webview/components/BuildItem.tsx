import React from "react";
import { useReorderDrag } from "../hooks/useDragDrop";

interface BuildItemProps {
  name: string;
  category: string;
  index: number;
  onRemove: () => void;
}

export function BuildItem({ name, category, index, onRemove }: BuildItemProps) {
  const { onDragStart, draggable } = useReorderDrag(name, category, index);

  return (
    <div
      className="build-item"
      draggable={draggable}
      onDragStart={onDragStart}
      data-build-item
    >
      <span className="build-item-icon">&#128204;</span>
      <span className="build-item-name">{name}</span>
      <button
        className="build-item-remove"
        onClick={onRemove}
        title={`Remove ${name}`}
        aria-label={`Remove ${name}`}
      >
        &#10005;
      </button>
    </div>
  );
}
