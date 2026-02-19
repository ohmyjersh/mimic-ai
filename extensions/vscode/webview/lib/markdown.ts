/**
 * Minimal markdown-to-HTML renderer for compose output.
 * Handles only the subset mimic compose produces: headings, paragraphs,
 * bold, italic, inline code, bullet lists, and horizontal rules.
 */
export function renderMarkdown(src: string): string {
  const lines = src.split("\n");
  const out: string[] = [];
  let inList = false;

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];

    // Horizontal rule
    if (/^---+\s*$/.test(line)) {
      if (inList) {
        out.push("</ul>");
        inList = false;
      }
      out.push("<hr>");
      continue;
    }

    // Heading (## only — that's what compose generates)
    const headingMatch = line.match(/^(#{1,3})\s+(.+)$/);
    if (headingMatch) {
      if (inList) {
        out.push("</ul>");
        inList = false;
      }
      const level = headingMatch[1].length;
      out.push(`<h${level}>${inlineFormat(headingMatch[2])}</h${level}>`);
      continue;
    }

    // Bullet list item
    if (/^[-*]\s+/.test(line)) {
      if (!inList) {
        out.push("<ul>");
        inList = true;
      }
      out.push(`<li>${inlineFormat(line.replace(/^[-*]\s+/, ""))}</li>`);
      continue;
    }

    // Close list if we hit a non-list line
    if (inList) {
      out.push("</ul>");
      inList = false;
    }

    // Empty line
    if (line.trim() === "") {
      continue;
    }

    // Paragraph
    out.push(`<p>${inlineFormat(line)}</p>`);
  }

  if (inList) {
    out.push("</ul>");
  }

  return out.join("\n");
}

function inlineFormat(text: string): string {
  return text
    // Inline code (before bold/italic to avoid conflicts)
    .replace(/`([^`]+)`/g, "<code>$1</code>")
    // Bold
    .replace(/\*\*([^*]+)\*\*/g, "<strong>$1</strong>")
    // Italic
    .replace(/\*([^*]+)\*/g, "<em>$1</em>")
    // Escape any remaining HTML
    .replace(/</g, (match, offset, str) => {
      // Don't escape our own tags
      if (
        str.substring(offset).startsWith("<code>") ||
        str.substring(offset).startsWith("</code>") ||
        str.substring(offset).startsWith("<strong>") ||
        str.substring(offset).startsWith("</strong>") ||
        str.substring(offset).startsWith("<em>") ||
        str.substring(offset).startsWith("</em>")
      ) {
        return match;
      }
      return "&lt;";
    });
}

export function countWords(text: string): number {
  return text
    .replace(/[#*`\-_]/g, "")
    .split(/\s+/)
    .filter((w) => w.length > 0).length;
}
