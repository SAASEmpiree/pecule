<script lang="ts" generics="T extends string | number">
  let {
    options,
    value = $bindable(),
    ariaLabel = "",
    size = "md",
    onchange,
  }: {
    options: { value: T; label: string }[];
    value: T;
    ariaLabel?: string;
    size?: "sm" | "md";
    onchange?: (v: T) => void;
  } = $props();

  function select(v: T) {
    value = v;
    onchange?.(v);
  }
</script>

<div class="seg" class:sm={size === "sm"} role="group" aria-label={ariaLabel}>
  {#each options as o (o.value)}
    <button
      type="button"
      class:active={o.value === value}
      aria-pressed={o.value === value}
      onclick={() => select(o.value)}>{o.label}</button
    >
  {/each}
</div>

<style>
  .seg {
    display: inline-flex;
    gap: 2px;
    padding: 3px;
    background: var(--surface-2);
    border: 1px solid var(--line);
    border-radius: var(--radius-sm);
  }
  button {
    border: 0;
    background: transparent;
    color: var(--muted);
    padding: 7px 14px;
    border-radius: 7px;
    font-size: var(--fs-small);
    font-weight: 500;
    transition: color var(--dur-fast) var(--ease),
      background var(--dur-fast) var(--ease);
  }
  .seg.sm button {
    padding: 5px 10px;
  }
  button:hover {
    color: var(--ink);
  }
  button.active {
    background: var(--surface);
    color: var(--ink);
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.25);
  }
</style>
