import React, { useMemo, useState, useRef, useCallback } from "react";
import { FragmentInfo, ResolveNode, ResolveEdge } from "../state";
import { forceLayout, getCategoryColor, LayoutNode } from "../lib/graph-layout";

interface GraphViewProps {
  personas: FragmentInfo[];
  graphPersona: string | null;
  nodes: ResolveNode[];
  edges: ResolveEdge[];
  selectedFragments: Set<string>;
  onDrillDown: (persona: string) => void;
  onBack: () => void;
  onToggle: (name: string, category: string) => void;
}

const NODE_RADIUS = 6;
const PERSONA_RADIUS = 20;

const CATEGORY_LABELS: [string, string][] = [
  ["persona", "Personas"],
  ["skill", "Skills"],
  ["context", "Contexts"],
  ["tone", "Tones"],
  ["constraint", "Constraints"],
];

function computeZoneLabel(
  categoryNodes: LayoutNode[]
): { x: number; y: number } | null {
  if (categoryNodes.length === 0) return null;
  let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity;
  for (const n of categoryNodes) {
    if (n.x < minX) minX = n.x;
    if (n.y < minY) minY = n.y;
    if (n.x > maxX) maxX = n.x;
    if (n.y > maxY) maxY = n.y;
  }
  return { x: (minX + maxX) / 2, y: minY - 24 };
}

// --- Persona Overview (root view) ---

function PersonaOverview({
  personas,
  selectedFragments,
  onDrillDown,
}: {
  personas: FragmentInfo[];
  selectedFragments: Set<string>;
  onDrillDown: (name: string) => void;
}) {
  // Lay out personas in a grid
  const cols = Math.ceil(Math.sqrt(personas.length * 1.5));
  const cellW = 160;
  const cellH = 100;
  const width = cols * cellW + 80;
  const rows = Math.ceil(personas.length / cols);
  const height = rows * cellH + 80;

  return (
    <div className="graph-container">
      <svg
        className="graph-svg"
        viewBox={`0 0 ${width} ${height}`}
      >
        {personas.map((p, i) => {
          const col = i % cols;
          const row = Math.floor(i / cols);
          const cx = 40 + col * cellW + cellW / 2;
          const cy = 40 + row * cellH + cellH / 2;
          const isSelected = selectedFragments.has(p.name);
          return (
            <g
              key={p.name}
              className={`graph-node persona-node${isSelected ? " selected" : ""}`}
              transform={`translate(${cx}, ${cy})`}
              onClick={() => onDrillDown(p.name)}
              style={{ cursor: "pointer" }}
            >
              <circle
                r={PERSONA_RADIUS}
                fill={getCategoryColor("persona")}
                stroke={isSelected ? "var(--studio-card-selected-border)" : "rgba(255,255,255,0.1)"}
                strokeWidth={isSelected ? 3 : 1}
              />
              <text
                dy={PERSONA_RADIUS + 16}
                textAnchor="middle"
                fontSize="11"
                fontWeight="600"
              >
                {p.name}
              </text>
            </g>
          );
        })}
      </svg>

      <div className="graph-hint">
        Click a persona to explore its fragment relationships
      </div>
    </div>
  );
}

// --- Drilled-in Graph ---

function DrillDownGraph({
  personaName,
  nodes,
  edges,
  selectedFragments,
  onBack,
  onToggle,
}: {
  personaName: string;
  nodes: ResolveNode[];
  edges: ResolveEdge[];
  selectedFragments: Set<string>;
  onBack: () => void;
  onToggle: (name: string, category: string) => void;
}) {
  const [zoom, setZoom] = useState(1);
  const [pan, setPan] = useState({ x: 0, y: 0 });
  const dragging = useRef(false);
  const lastPos = useRef({ x: 0, y: 0 });

  const layout = useMemo(() => {
    if (nodes.length === 0) return null;
    return forceLayout(
      nodes.map((n) => ({ id: n.name, category: n.category })),
      edges.map((e) => ({ source: e.from, target: e.to })),
      0,
      0
    );
  }, [nodes, edges]);

  const zoneLabels = useMemo(() => {
    if (!layout) return [];
    const result: { category: string; label: string; x: number; y: number }[] = [];
    for (const [cat, label] of CATEGORY_LABELS) {
      const catNodes = layout.nodes.filter((n) => n.category === cat);
      const pos = computeZoneLabel(catNodes);
      if (pos) {
        result.push({ category: cat, label, ...pos });
      }
    }
    return result;
  }, [layout]);

  const handleWheel = useCallback((e: React.WheelEvent) => {
    e.preventDefault();
    setZoom((z) => Math.max(0.1, Math.min(3, z - e.deltaY * 0.001)));
  }, []);

  const handleMouseDown = useCallback((e: React.MouseEvent) => {
    if (e.target === e.currentTarget || (e.target as Element).tagName === "svg") {
      dragging.current = true;
      lastPos.current = { x: e.clientX, y: e.clientY };
    }
  }, []);

  const handleMouseMove = useCallback((e: React.MouseEvent) => {
    if (!dragging.current) return;
    const dx = e.clientX - lastPos.current.x;
    const dy = e.clientY - lastPos.current.y;
    lastPos.current = { x: e.clientX, y: e.clientY };
    setPan((p) => ({ x: p.x + dx, y: p.y + dy }));
  }, []);

  const handleMouseUp = useCallback(() => {
    dragging.current = false;
  }, []);

  // Loading state while resolve data comes back
  if (nodes.length === 0) {
    return (
      <div className="graph-container">
        <div className="graph-back-bar">
          <button className="graph-back-btn" onClick={onBack}>
            &#8592; All Personas
          </button>
          <span className="graph-back-title">{personaName}</span>
        </div>
        <div className="empty-state">
          <div className="loading-skeleton">
            <div className="skeleton-line" />
            <div className="skeleton-line" />
            <div className="skeleton-line" />
          </div>
        </div>
      </div>
    );
  }

  if (!layout) return null;

  const { width: vw, height: vh } = layout;
  const nodeMap = new Map(layout.nodes.map((n) => [n.id, n]));

  return (
    <div className="graph-container">
      <div className="graph-back-bar">
        <button className="graph-back-btn" onClick={onBack}>
          &#8592; All Personas
        </button>
        <span className="graph-back-title">{personaName}</span>
        <span className="graph-back-count">{nodes.length} fragments</span>
      </div>
      <svg
        className="graph-svg"
        viewBox={`0 0 ${vw} ${vh}`}
        onWheel={handleWheel}
        onMouseDown={handleMouseDown}
        onMouseMove={handleMouseMove}
        onMouseUp={handleMouseUp}
        onMouseLeave={handleMouseUp}
      >
        <g
          transform={`translate(${pan.x}, ${pan.y}) scale(${zoom})`}
          style={{ transformOrigin: `${vw / 2}px ${vh / 2}px` }}
        >
          {/* Edges */}
          {layout.edges.map((edge, i) => {
            const src = nodeMap.get(edge.source);
            const tgt = nodeMap.get(edge.target);
            if (!src || !tgt) return null;
            return (
              <line
                key={i}
                className="graph-edge"
                x1={src.x}
                y1={src.y}
                x2={tgt.x}
                y2={tgt.y}
              />
            );
          })}

          {/* Category zone labels */}
          {zoneLabels.map(({ category, label, x, y }) => (
            <text
              key={category}
              className="graph-zone-label"
              x={x}
              y={y}
              textAnchor="middle"
              fill={getCategoryColor(category)}
            >
              {label}
            </text>
          ))}

          {/* Nodes */}
          {layout.nodes.map((node) => {
            const isSelected = selectedFragments.has(node.id);
            const originalNode = nodes.find((n) => n.name === node.id);
            const category = originalNode?.category ?? node.category;
            const isPersona = category === "persona";
            const r = isPersona ? PERSONA_RADIUS / 2 : NODE_RADIUS;
            return (
              <g
                key={node.id}
                className={`graph-node${isSelected ? " selected" : ""}`}
                transform={`translate(${node.x}, ${node.y})`}
                onClick={() => onToggle(node.id, category)}
              >
                <circle
                  r={r}
                  fill={getCategoryColor(category)}
                  stroke={isSelected ? "var(--studio-card-selected-border)" : "none"}
                  strokeWidth={isSelected ? 3 : 0}
                />
                <text
                  dy={r + 12}
                  textAnchor="middle"
                  fontSize="9"
                  fontWeight={isPersona ? "700" : "400"}
                >
                  {node.id}
                </text>
              </g>
            );
          })}
        </g>
      </svg>

      <div className="graph-legend">
        {CATEGORY_LABELS.map(([cat, label]) => (
          <div key={cat} className="legend-item">
            <span
              className="legend-dot"
              style={{ background: getCategoryColor(cat) }}
            />
            {label}
          </div>
        ))}
      </div>
    </div>
  );
}

// --- Main GraphView ---

export function GraphView({
  personas,
  graphPersona,
  nodes,
  edges,
  selectedFragments,
  onDrillDown,
  onBack,
  onToggle,
}: GraphViewProps) {
  if (graphPersona) {
    return (
      <DrillDownGraph
        personaName={graphPersona}
        nodes={nodes}
        edges={edges}
        selectedFragments={selectedFragments}
        onBack={onBack}
        onToggle={onToggle}
      />
    );
  }

  if (personas.length === 0) {
    return (
      <div className="graph-container">
        <div className="empty-state">
          <div className="empty-state-icon">&#9734;</div>
          <div className="empty-state-title">Loading Personas</div>
          <div className="empty-state-text">
            Waiting for fragment data...
          </div>
        </div>
      </div>
    );
  }

  return (
    <PersonaOverview
      personas={personas}
      selectedFragments={selectedFragments}
      onDrillDown={onDrillDown}
    />
  );
}
