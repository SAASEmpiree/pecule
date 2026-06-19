<script lang="ts">
  import type { Snippet } from "svelte";

  let {
    title,
    subtitle,
    children,
    actions,
    pad = true,
    class: klass = "",
  }: {
    title?: string;
    subtitle?: string;
    children: Snippet;
    actions?: Snippet;
    pad?: boolean;
    class?: string;
  } = $props();
</script>

<section class="card {klass}" class:pad>
  {#if title || subtitle || actions}
    <header>
      <div class="titles">
        {#if title}<h2 class="h2">{title}</h2>{/if}
        {#if subtitle}<p class="small muted">{subtitle}</p>{/if}
      </div>
      {#if actions}<div class="actions">{@render actions()}</div>{/if}
    </header>
  {/if}
  {@render children()}
</section>

<style>
  .card.pad {
    padding: var(--space-6);
  }
  header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: var(--space-4);
    margin-bottom: var(--space-4);
  }
  .titles {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }
  .actions {
    flex-shrink: 0;
  }
</style>
