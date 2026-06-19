<script lang="ts">
  type Tone = "neutral" | "gain" | "loss" | "accent";
  let {
    label,
    value,
    sub,
    tone = "neutral",
  }: {
    label: string;
    value: string;
    sub?: string;
    tone?: Tone;
  } = $props();
</script>

<div class="stat">
  <span class="small muted label">{label}</span>
  <span
    class="value mono"
    class:gain={tone === "gain"}
    class:loss={tone === "loss"}
    class:accent={tone === "accent"}>{value}</span
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
  .value.accent {
    color: var(--accent);
  }
</style>
