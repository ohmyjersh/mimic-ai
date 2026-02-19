export interface LayoutNode {
  id: string;
  category: string;
  x: number;
  y: number;
  vx: number;
  vy: number;
}

export interface LayoutEdge {
  source: string;
  target: string;
}

export interface LayoutResult {
  nodes: LayoutNode[];
  edges: LayoutEdge[];
  width: number;
  height: number;
}

// Category zone anchors arranged in a pentagon around center.
// Each category's nodes are pulled toward their anchor.
const CATEGORY_ZONES: Record<string, { ax: number; ay: number }> = {
  persona:    { ax: 0.50, ay: 0.10 }, // top center
  skill:      { ax: 0.85, ay: 0.38 }, // right
  context:    { ax: 0.72, ay: 0.82 }, // bottom right
  tone:       { ax: 0.28, ay: 0.82 }, // bottom left
  constraint: { ax: 0.15, ay: 0.38 }, // left
};

// Approximate text width in SVG pixels for a 9px sans-serif font.
// Average char width ~5.4px at 9px font size. Add node radius padding on each side.
const CHAR_WIDTH = 5.4;
const FONT_SIZE = 9;
const NODE_PAD = 8; // extra padding beyond the text half-width

function estimateTextWidth(label: string): number {
  return label.length * CHAR_WIDTH;
}

/**
 * Force-directed graph layout with category clustering.
 *
 * - Minimum distance between nodes is derived from their label widths
 * - Nodes repel proportional to their combined text footprint
 * - Each category has a "home zone" that pulls its nodes into a cluster
 * - Canvas scales with total text area so large/long-named graphs have room
 */
export function forceLayout(
  nodeIds: { id: string; category: string }[],
  edges: { source: string; target: string }[],
  _width: number,
  _height: number,
  iterations: number = 150
): LayoutResult {
  // Precompute half-widths (radius of each node's text footprint)
  const halfWidths = new Map<string, number>();
  let totalTextArea = 0;
  for (const n of nodeIds) {
    const hw = estimateTextWidth(n.id) / 2 + NODE_PAD;
    halfWidths.set(n.id, hw);
    // Each node needs a box of (2*hw) x LINE_HEIGHT to not overlap
    totalTextArea += (2 * hw) * (FONT_SIZE + 20);
  }

  // Scale canvas to total text area with generous padding (3x for spacing)
  const aspect = 4 / 3;
  const height = Math.max(800, Math.sqrt((totalTextArea * 3) / aspect));
  const width = Math.max(1200, height * aspect);
  const centerX = width / 2;
  const centerY = height / 2;

  // Initialize nodes near their category zone
  const catIndices = new Map<string, number>();
  const catCounts = new Map<string, number>();
  for (const n of nodeIds) {
    catCounts.set(n.category, (catCounts.get(n.category) ?? 0) + 1);
  }

  const nodes: LayoutNode[] = nodeIds.map((n) => {
    const zone = CATEGORY_ZONES[n.category] ?? { ax: 0.5, ay: 0.5 };
    const zoneX = zone.ax * width;
    const zoneY = zone.ay * height;
    const catIndex = catIndices.get(n.category) ?? 0;
    catIndices.set(n.category, catIndex + 1);
    const catCount = catCounts.get(n.category) ?? 1;
    const angle = (2 * Math.PI * catIndex) / Math.max(catCount, 1);
    const spread = Math.min(width, height) * 0.12 * Math.sqrt((catIndex + 1) / catCount);
    return {
      id: n.id,
      category: n.category,
      x: zoneX + spread * Math.cos(angle),
      y: zoneY + spread * Math.sin(angle),
      vx: 0,
      vy: 0,
    };
  });

  if (nodes.length <= 1) {
    if (nodes.length === 1) {
      nodes[0].x = centerX;
      nodes[0].y = centerY;
    }
    return { nodes, edges, width, height };
  }

  const nodeMap = new Map<string, LayoutNode>();
  for (const n of nodes) {
    nodeMap.set(n.id, n);
  }

  const repulsion = 15000;
  const attraction = 0.005;
  const damping = 0.85;
  const centerGravity = 0.005;
  const zoneGravity = 0.04;
  const verticalSpacing = FONT_SIZE + 18; // min vertical gap for label rows

  for (let iter = 0; iter < iterations; iter++) {
    const temp = 1 - iter / iterations;

    // Repulsive forces between all node pairs, scaled by text size
    for (let i = 0; i < nodes.length; i++) {
      for (let j = i + 1; j < nodes.length; j++) {
        const a = nodes[i];
        const b = nodes[j];

        // Per-pair minimum distance based on their combined text half-widths
        const hwA = halfWidths.get(a.id) ?? 30;
        const hwB = halfWidths.get(b.id) ?? 30;
        const pairMinDist = Math.max(hwA + hwB, verticalSpacing);

        let dx = a.x - b.x;
        let dy = a.y - b.y;
        let dist = Math.sqrt(dx * dx + dy * dy);
        if (dist < pairMinDist) {
          // Nodes overlap — apply strong separation
          if (dist < 1) {
            dx = (Math.random() - 0.5) * pairMinDist;
            dy = (Math.random() - 0.5) * pairMinDist;
            dist = pairMinDist;
          }
        }

        const categoryScale = a.category === b.category ? 0.6 : 1.0;
        // Scale repulsion by pair size so wider labels push harder
        const sizeScale = pairMinDist / 60;
        const force = (repulsion * categoryScale * sizeScale * temp) / (dist * dist);
        const fx = (dx / dist) * force;
        const fy = (dy / dist) * force;
        a.vx += fx;
        a.vy += fy;
        b.vx -= fx;
        b.vy -= fy;
      }
    }

    // Attractive forces along edges
    for (const edge of edges) {
      const a = nodeMap.get(edge.source);
      const b = nodeMap.get(edge.target);
      if (!a || !b) continue;

      const dx = b.x - a.x;
      const dy = b.y - a.y;
      const dist = Math.sqrt(dx * dx + dy * dy);
      if (dist < 1) continue;

      const force = dist * attraction * temp;
      const fx = (dx / dist) * force;
      const fy = (dy / dist) * force;
      a.vx += fx;
      a.vy += fy;
      b.vx -= fx;
      b.vy -= fy;
    }

    // Category zone gravity
    for (const node of nodes) {
      const zone = CATEGORY_ZONES[node.category] ?? { ax: 0.5, ay: 0.5 };
      const zoneX = zone.ax * width;
      const zoneY = zone.ay * height;
      node.vx += (zoneX - node.x) * zoneGravity * temp;
      node.vy += (zoneY - node.y) * zoneGravity * temp;
    }

    // Weak global center gravity
    for (const node of nodes) {
      node.vx += (centerX - node.x) * centerGravity * temp;
      node.vy += (centerY - node.y) * centerGravity * temp;
    }

    // Apply velocities with damping
    for (const node of nodes) {
      node.vx *= damping;
      node.vy *= damping;
      node.x += node.vx;
      node.y += node.vy;

      const pad = 60;
      node.x = Math.max(pad, Math.min(width - pad, node.x));
      node.y = Math.max(pad, Math.min(height - pad, node.y));
    }
  }

  return { nodes, edges, width, height };
}

const CATEGORY_COLORS: Record<string, string> = {
  persona: "var(--cat-persona)",
  skill: "var(--cat-skill)",
  context: "var(--cat-context)",
  tone: "var(--cat-tone)",
  constraint: "var(--cat-constraint)",
};

export function getCategoryColor(category: string): string {
  return CATEGORY_COLORS[category] ?? "var(--studio-muted)";
}
