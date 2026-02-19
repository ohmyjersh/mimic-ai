import React from "react";
import { FragmentInfo } from "../state";
import { usePaletteDrag } from "../hooks/useDragDrop";

interface PaletteItemProps {
  fragment: FragmentInfo;
  category: string;
  isSelected: boolean;
  isRecommended: boolean;
  onAdd: () => void;
  onRemove: () => void;
}

export function PaletteItem({
  fragment,
  category,
  isSelected,
  isRecommended,
  onAdd,
  onRemove,
}: PaletteItemProps) {
  const { onDragStart, draggable } = usePaletteDrag(fragment.name, category);

  const handleClick = isSelected ? onRemove : onAdd;

  const classes = [
    "palette-item",
    isSelected ? "selected" : "",
    isRecommended && !isSelected ? "recommended" : "",
  ]
    .filter(Boolean)
    .join(" ");

  return (
    <div
      className={classes}
      draggable={draggable && !isSelected}
      onDragStart={onDragStart}
      onClick={handleClick}
      title={fragment.description}
      role="button"
      tabIndex={0}
      onKeyDown={(e) => {
        if (e.key === "Enter" || e.key === " ") {
          e.preventDefault();
          handleClick();
        }
      }}
    >
      {isRecommended && !isSelected && <span className="palette-item-star">&#9733;</span>}
      {isSelected && <span className="palette-item-check">&#10003;</span>}
      <span className="palette-item-name">{fragment.name}</span>
      {fragment.level && <span className="palette-item-badge">{fragment.level}</span>}
    </div>
  );
}
