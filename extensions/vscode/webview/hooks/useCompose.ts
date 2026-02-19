import { useEffect, useRef } from "react";
import { usePostMessage } from "./useVscodeApi";

export function useCompose(
  persona: string | null,
  skills: string[],
  contexts: string[],
  tones: string[],
  constraints: string[],
  onComposing: () => void
) {
  const postMessage = usePostMessage();
  const timerRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  useEffect(() => {
    if (timerRef.current) {
      clearTimeout(timerRef.current);
    }

    if (!persona) {
      return;
    }

    onComposing();

    timerRef.current = setTimeout(() => {
      postMessage({
        type: "compose",
        persona,
        skills,
        contexts,
        tones,
        constraints,
      });
    }, 300);

    return () => {
      if (timerRef.current) {
        clearTimeout(timerRef.current);
      }
    };
  }, [persona, skills, contexts, tones, constraints, postMessage, onComposing]);
}
