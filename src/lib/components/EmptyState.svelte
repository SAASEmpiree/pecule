<script lang="ts">
  import type { Component } from "svelte";

  let {
    icon,
    title,
    body,
    badge,
  }: {
    icon?: Component<any>;
    title: string;
    body: string;
    badge?: string;
  } = $props();
</script>

<div class="empty card pad">
  {#if icon}
    {@const Icon = icon}
    <div class="ic"><Icon size={26} aria-hidden="true" /></div>
  {/if}
  <div class="txt">
    <h2 class="h2">
      {title}{#if badge}<span class="badge">{badge}</span>{/if}
    </h2>
    <p class="muted">{body}</p>
  </div>
</div>

<style>
  .empty {
    display: flex;
    gap: var(--space-4);
    align-items: flex-start;
  }
  .ic {
    flex-shrink: 0;
    width: 48px;
    height: 48px;
    display: grid;
    place-items: center;
    border-radius: var(--radius-md);
    background: var(--accent-soft);
    color: var(--accent);
  }
  .txt {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }
  .badge {
    margin-left: var(--space-2);
    vertical-align: middle;
    background: var(--soon-soft);
    color: var(--soon);
    padding: 2px 8px;
    border-radius: var(--radius-pill);
    font-size: 11px;
    font-weight: 600;
  }
  p {
    max-width: 60ch;
    line-height: 1.5;
  }
</style>
