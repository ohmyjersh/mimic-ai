import React from "react";

interface RecommendationBarProps {
  recommendations: { name: string; category: string }[];
  onAdd: (name: string, category: string) => void;
}

export function RecommendationBar({ recommendations, onAdd }: RecommendationBarProps) {
  if (recommendations.length === 0) return null;

  return (
    <div className="recommendation-bar">
      <span className="recommendation-label">&#9733; Suggested:</span>
      {recommendations.map(({ name, category }) => (
        <button
          key={`${category}-${name}`}
          className="recommendation-chip"
          onClick={() => onAdd(name, category)}
          title={`Add ${name}`}
        >
          {name}
        </button>
      ))}
    </div>
  );
}
