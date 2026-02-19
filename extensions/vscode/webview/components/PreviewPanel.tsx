import React from "react";
import { renderMarkdown, countWords } from "../lib/markdown";

interface PreviewPanelProps {
  markdown: string;
  isComposing: boolean;
  hasPersona: boolean;
}

export function PreviewPanel({ markdown, isComposing, hasPersona }: PreviewPanelProps) {
  if (!hasPersona) {
    return (
      <div className="preview-panel">
        <div className="empty-state">
          <div className="empty-state-icon">&#9998;</div>
          <div className="empty-state-title">Build Your Prompt</div>
          <div className="empty-state-text">
            Drag fragments from the palette into the build zone, or click to add them.
            Start with a persona, then add skills, contexts, tones, and constraints.
          </div>
        </div>
      </div>
    );
  }

  if (isComposing && !markdown) {
    return (
      <div className="preview-panel">
        <div className="preview-content">
          <div className="loading-skeleton">
            <div className="skeleton-line" />
            <div className="skeleton-line" />
            <div className="skeleton-line" />
            <div className="skeleton-line" />
            <div className="skeleton-line" />
          </div>
        </div>
      </div>
    );
  }

  const html = renderMarkdown(markdown);
  const words = countWords(markdown);

  return (
    <div className="preview-panel">
      <div className="preview-content">
        <div
          className="markdown-preview"
          dangerouslySetInnerHTML={{ __html: html }}
        />
      </div>
      <div className="preview-footer">{words} words</div>
    </div>
  );
}
