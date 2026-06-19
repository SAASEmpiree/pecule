<script lang="ts">
  import { Tween } from "svelte/motion";
  import { cubicOut } from "svelte/easing";

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

  const tween = new Tween(value, {
    duration: reduce ? 0 : duration,
    easing: cubicOut,
  });

  $effect(() => {
    tween.target = value;
  });
</script>

<span class="tnum">{format(tween.current)}</span>
