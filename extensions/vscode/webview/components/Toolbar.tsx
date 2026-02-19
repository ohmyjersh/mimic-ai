import React, { useState } from "react";
import { usePostMessage } from "../hooks/useVscodeApi";
import { TemplateSelections, SavedTemplate } from "../state";

interface ToolbarProps {
  markdown: string;
  selections: TemplateSelections;
  templates: SavedTemplate[];
  hasPersona: boolean;
}

export function Toolbar({ markdown, selections, templates, hasPersona }: ToolbarProps) {
  const postMessage = usePostMessage();
  const [showSaveModal, setShowSaveModal] = useState(false);
  const [templateName, setTemplateName] = useState("");

  const handleCopy = () => {
    if (markdown) {
      postMessage({ type: "copyToClipboard", text: markdown });
    }
  };

  const handleInsert = () => {
    if (markdown) {
      postMessage({ type: "insertIntoEditor", text: markdown });
    }
  };

  const handleSave = () => {
    if (templateName.trim()) {
      postMessage({
        type: "saveTemplate",
        name: templateName.trim(),
        selections,
      });
      setTemplateName("");
      setShowSaveModal(false);
    }
  };

  return (
    <>
      <div className="preview-toolbar">
        <button
          className="toolbar-btn"
          onClick={handleCopy}
          disabled={!markdown}
          title="Copy prompt to clipboard"
        >
          Copy
        </button>
        <button
          className="toolbar-btn primary"
          onClick={handleInsert}
          disabled={!markdown}
          title="Insert prompt into active editor"
        >
          Insert
        </button>
        <div className="toolbar-spacer" />
        <button
          className="toolbar-btn"
          onClick={() => setShowSaveModal(true)}
          disabled={!hasPersona}
          title="Save current selections as a template"
        >
          Save Template
        </button>
      </div>

      {showSaveModal && (
        <div className="modal-overlay" onClick={() => setShowSaveModal(false)}>
          <div className="modal" onClick={(e) => e.stopPropagation()}>
            <h3>Save Template</h3>
            <input
              type="text"
              placeholder="Template name..."
              value={templateName}
              onChange={(e) => setTemplateName(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === "Enter") handleSave();
                if (e.key === "Escape") setShowSaveModal(false);
              }}
              autoFocus
            />
            <div className="modal-actions">
              <button
                className="toolbar-btn"
                onClick={() => setShowSaveModal(false)}
              >
                Cancel
              </button>
              <button
                className="toolbar-btn primary"
                onClick={handleSave}
                disabled={!templateName.trim()}
              >
                Save
              </button>
            </div>
          </div>
        </div>
      )}
    </>
  );
}
