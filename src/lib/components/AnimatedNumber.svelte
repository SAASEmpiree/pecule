<script lang="ts">
  import { Tween } from "svelte/motion";
  import { cubicOut } from "svelte/easing";
  import { untrack } from "svelte";

  let {
    value,
    format = (n: number) => String(Math.round(n)),
    duration = 480,
  }: {
    value: number;
    format?: (n: number) => string;
    duration?: number;
  } = $props();

  const reduce =
    typeof window !== "undefined" &&
    window.matchMedia?.("(prefers-reduced-motion: reduce)").matches;

  // Lecture initiale unique de `value`/`duration` hors suivi réactif : les mises
  // à jour passent par l'effet ci-dessous (silence `state_referenced_locally`).
  const tween = untrack(
    () => new Tween(value, { duration: reduce ? 0 : duration, easing: cubicOut }),
  );

  $effect(() => {
    tween.target = value;
  });
</script>

<span class="tnum">{format(tween.current)}</span>
