import React, { useCallback, useRef } from "react";

export interface DragData {
  name: string;
  category: string;
}

const MIME_FRAGMENT = "application/mimic-fragment";
const MIME_REORDER = "application/mimic-reorder";

// Module-level ref to work around VS Code webview's Chromium security
// restriction where dataTransfer.getData() returns empty during dragover.
let currentDrag: DragData | null = null;
let currentReorder: (DragData & { index: number }) | null = null;

export function getCurrentDragCategory(): string | null {
  return currentDrag?.category ?? currentReorder?.category ?? null;
}

export function usePaletteDrag(name: string, category: string) {
  const onDragStart = useCallback(
    (e: React.DragEvent) => {
      const data: DragData = { name, category };
      e.dataTransfer.setData(MIME_FRAGMENT, JSON.stringify(data));
      e.dataTransfer.effectAllowed = "copy";
      currentDrag = data;
      currentReorder = null;
    },
    [name, category]
  );

  return { onDragStart, draggable: true };
}

export function useReorderDrag(name: string, category: string, index: number) {
  const onDragStart = useCallback(
    (e: React.DragEvent) => {
      const data = { name, category, index };
      e.dataTransfer.setData(MIME_REORDER, JSON.stringify(data));
      e.dataTransfer.effectAllowed = "move";
      currentReorder = data;
      currentDrag = null;
    },
    [name, category, index]
  );

  return { onDragStart, draggable: true };
}

export interface DropHandlers {
  onDragOver: (e: React.DragEvent) => void;
  onDragLeave: (e: React.DragEvent) => void;
  onDrop: (e: React.DragEvent) => void;
}

export function useSlotDrop(
  category: string,
  onAdd: (name: string) => void,
  onReorder: (names: string[]) => void,
  currentItems: string[]
): DropHandlers & { isOver: boolean } {
  const isOverRef = useRef(false);
  const [isOver, setIsOver] = React.useState(false);

  const onDragOver = useCallback(
    (e: React.DragEvent) => {
      // Check if this is a valid drop for this category
      const isFragment = e.dataTransfer.types.includes(MIME_FRAGMENT) && currentDrag?.category === category;
      const isReorder = e.dataTransfer.types.includes(MIME_REORDER) && currentReorder?.category === category;

      if (isFragment || isReorder) {
        e.preventDefault();
        e.dataTransfer.dropEffect = isFragment ? "copy" : "move";
        if (!isOverRef.current) {
          isOverRef.current = true;
          setIsOver(true);
        }
      }
    },
    [category]
  );

  const onDragLeave = useCallback((e: React.DragEvent) => {
    // Only trigger when leaving the slot itself, not children
    const rect = e.currentTarget.getBoundingClientRect();
    const { clientX, clientY } = e;
    if (
      clientX < rect.left ||
      clientX > rect.right ||
      clientY < rect.top ||
      clientY > rect.bottom
    ) {
      isOverRef.current = false;
      setIsOver(false);
    }
  }, []);

  const onDrop = useCallback(
    (e: React.DragEvent) => {
      e.preventDefault();
      isOverRef.current = false;
      setIsOver(false);

      // Handle palette -> build zone drop
      if (e.dataTransfer.types.includes(MIME_FRAGMENT) && currentDrag) {
        if (currentDrag.category === category) {
          onAdd(currentDrag.name);
        }
        currentDrag = null;
        return;
      }

      // Handle reorder drop
      if (e.dataTransfer.types.includes(MIME_REORDER) && currentReorder) {
        if (currentReorder.category === category) {
          const draggedName = currentReorder.name;
          const fromIndex = currentReorder.index;

          // Calculate target index from mouse position
          const items = e.currentTarget.querySelectorAll("[data-build-item]");
          let targetIndex = currentItems.length;

          for (let i = 0; i < items.length; i++) {
            const rect = items[i].getBoundingClientRect();
            const midY = rect.top + rect.height / 2;
            if (e.clientY < midY) {
              targetIndex = i;
              break;
            }
          }

          // Build new order
          const newOrder = currentItems.filter((n) => n !== draggedName);
          const adjustedIndex = targetIndex > fromIndex ? targetIndex - 1 : targetIndex;
          newOrder.splice(Math.min(adjustedIndex, newOrder.length), 0, draggedName);
          onReorder(newOrder);
        }
        currentReorder = null;
      }
    },
    [category, onAdd, onReorder, currentItems]
  );

  return { onDragOver, onDragLeave, onDrop, isOver };
}
