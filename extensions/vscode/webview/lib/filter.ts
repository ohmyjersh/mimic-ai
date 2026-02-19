import { FragmentInfo } from "../state";

export function filterFragments(fragments: FragmentInfo[], query: string): FragmentInfo[] {
  if (!query) return fragments;
  const q = query.toLowerCase();
  return fragments.filter(
    (f) =>
      f.name.toLowerCase().includes(q) ||
      f.description.toLowerCase().includes(q) ||
      f.tags.some((t) => t.toLowerCase().includes(q))
  );
}
