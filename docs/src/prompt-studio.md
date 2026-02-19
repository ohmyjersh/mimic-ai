# Prompt Studio

Prompt Studio is a visual interface for composing persona prompts inside VS Code. It provides drag-and-drop fragment selection, live prompt preview, and a relationship graph -- all without leaving your editor.

## Opening Prompt Studio

Open the command palette (`Cmd+Shift+P` / `Ctrl+Shift+P`) and run:

```
Mimic: Open Prompt Studio
```

This opens a new editor tab with three panels: the Palette, the Build Zone, and the Preview.

## The Builder

The Builder tab is the default view. It has three columns:

### Palette (left)

The palette lists all available fragments organized into collapsible sections: Personas, Skills, Contexts, Tones, and Constraints. Each section header shows the total fragment count and how many are currently selected.

- **Click a section header** to expand or collapse it. Only one section is open at a time -- clicking a different header closes the current one and opens the new one. Clicking the open section's header collapses it entirely.
- **Search** within any open section using the search box at the top.
- **Click a fragment** to add it to the Build Zone. Selected fragments show a checkmark.
- **Click a selected fragment** again to remove it from the Build Zone.
- **Drag a fragment** from the palette into the matching Build Zone slot. The valid drop target highlights with a dashed border and a "+" badge.

Fragments marked with a star are recommended based on the selected persona.

### Build Zone (center)

The Build Zone shows five drop slots -- one for each fragment category. This is where you assemble your prompt.

- **Persona** accepts exactly one persona. Selecting a new one replaces the current selection.
- **Skills, Contexts, Tones, Constraints** accept multiple fragments. Drag to reorder them within a slot.
- **Remove** a fragment by clicking the X button on its chip.

When you drag a fragment, only the matching slot highlights as a valid drop target. Non-matching slots stay inert.

Below the slots, a **Recommendation Bar** appears when a persona is selected. It shows unselected fragments that pair well with the current persona. Click any recommendation to add it instantly.

### Preview (right)

The Preview panel shows the composed system prompt in real time. As you add, remove, or reorder fragments, the preview updates automatically (with a short debounce).

The footer shows the total word count of the composed prompt.

**Toolbar actions:**
- **Copy** -- copies the composed prompt to the clipboard.
- **Insert** -- inserts the prompt at the cursor position in the active editor. If no editor is open, creates a new markdown document.
- **Save Template** -- saves the current selection as a named template for reuse.
- **Load Template** -- restores a previously saved selection.

## The Graph

The Graph tab visualizes relationships between fragments as an interactive force-directed graph.

- **Nodes** represent fragments, colored by category.
- **Edges** show relationships: shared tags, skill groups, and group membership.
- **Click a node** to toggle its selection (same as clicking in the palette).
- **Zoom** with the scroll wheel.
- **Pan** by clicking and dragging the background.

The graph loads automatically when you switch to the Graph tab. If a persona is selected, the graph is anchored around that persona's related fragments. Without a persona, all fragments and their relationships are shown.

A color-coded legend at the bottom identifies each category.

## Templates

Templates save your current fragment selection so you can restore it later.

1. Build a prompt you like in the Builder tab.
2. Click **Save Template** in the toolbar.
3. Enter a name and save.
4. Later, click **Load Template** and select a saved template to restore that exact selection.

Templates are stored per-workspace in VS Code's extension storage.

## Sidebar

The extension also adds two sidebar views under the Mimic icon in the activity bar:

- **Fragments** -- a tree view of all available fragments grouped by category. Expanding a category shows each fragment with its description.
- **Templates** -- a list of saved templates with persona name and fragment count.

## Tips

- Start by selecting a persona. This populates recommendations and anchors the graph.
- Use the Graph tab to discover fragments you might not have considered -- look for clusters of related nodes.
- Save templates for common workflows (e.g., "code review as backend engineer" or "debugging with security focus").
- Project-local fragments from `.mimic/` appear alongside built-in defaults in the palette.
