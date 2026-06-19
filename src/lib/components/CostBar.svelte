<script lang="ts">
  let {
    segments,
    format,
    ariaLabel = "",
  }: {
    segments: { label: string; value: number; color: string }[];
    format: (n: number) => string;
    ariaLabel?: string;
  } = $props();

  const total = $derived(
    segments.reduce((s, x) => s + Math.max(x.value, 0), 0) || 1,
  );
</script>

<div class="costbar">
  <div class="bar" role="img" aria-label={ariaLabel}>
    {#each segments as s (s.label)}
      {#if s.value > 0}
        <div
          class="seg"
          style="width:{((s.value / total) * 100).toFixed(2)}%; background:{s.color}"
        ></div>
      {/if}
    {/each}
  </div>
  <ul class="legend">
    {#each segments as s (s.label)}
      {#if s.value > 0}
        <li>
          <span class="dot" style="background:{s.color}"></span>
          <span class="small muted">{s.label}</span>
          <span class="small mono val">{format(s.value)}</span>
        </li>
      {/if}
    {/each}
  </ul>
</div>

<style>
  .costbar {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }
  .bar {
    display: flex;
    height: 14px;
    border-radius: var(--radius-pill);
    overflow: hidden;
    background: var(--surface-2);
  }
  .seg {
    height: 100%;
  }
  .seg + .seg {
    box-shadow: -1px 0 0 var(--surface);
  }
  .legend {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-2) var(--space-6);
  }
  .legend li {
    display: flex;
    align-items: center;
    gap: var(--space-2);
  }
  .dot {
    width: 10px;
    height: 10px;
    border-radius: 3px;
    flex-shrink: 0;
  }
  .val {
    color: var(--ink);
    font-weight: 500;
  }
</style>
