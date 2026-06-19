<script lang="ts">
  type Tone = "neutral" | "gain" | "loss" | "accent" | "soon";
  let {
    label,
    value,
    sub,
    tone = "neutral",
    big = false,
  }: {
    label: string;
    value: string;
    sub?: string;
    tone?: Tone;
    big?: boolean;
  } = $props();
</script>

<div class="stat">
  <span class="small muted label">{label}</span>
  <span
    class="value mono"
    class:big
    class:gain={tone === "gain"}
    class:loss={tone === "loss"}
    class:accent={tone === "accent"}
    class:soon={tone === "soon"}>{value}</span
  >
  {#if sub}
    <span
      class="small sub"
      class:gain={tone === "gain"}
      class:loss={tone === "loss"}
      class:muted={tone === "neutral" || tone === "accent"}>{sub}</span
    >
  {/if}
</div>

<style>
  .stat {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
  .label {
    letter-spacing: 0.01em;
  }
  .value {
    font-size: 24px;
    font-weight: 500;
    line-height: 1.1;
  }
  .value.big {
    font-size: var(--fs-display);
    font-weight: 600;
    letter-spacing: -0.01em;
  }
</style>
