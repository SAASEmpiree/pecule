<script lang="ts">
  let { size = 22 }: { size?: number } = $props();
</script>

<svg
  width={size}
  height={size}
  viewBox="0 0 24 24"
  fill="none"
  aria-hidden="true"
  class="logo"
>
  <rect x="0.5" y="0.5" width="23" height="23" rx="7" class="bg" />
  <path
    d="M4 17.5 C 8.5 17.5, 9.5 7.5, 13.5 7 S 18.5 5, 20 4.5"
    class="stroke"
    stroke-width="2.2"
    stroke-linecap="round"
  />
  <circle cx="20" cy="4.5" r="1.9" class="dot" />
</svg>

<style>
  .bg {
    fill: var(--accent-soft);
    stroke: var(--line);
  }
  .stroke {
    stroke: var(--accent);
    fill: none;
  }
  .dot {
    fill: var(--accent);
  }
</style>
