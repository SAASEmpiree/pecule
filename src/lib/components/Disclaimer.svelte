<script lang="ts">
  import { Info } from "@lucide/svelte";
  import { t } from "$lib/i18n";

  let { long = false }: { long?: boolean } = $props();
</script>

<p class="disclaimer small muted">
  <Info size={14} aria-hidden="true" />
  <span>{long ? t("common.notAdviceLong") : t("common.notAdvice")}</span>
</p>

<style>
  .disclaimer {
    display: flex;
    align-items: flex-start;
    gap: var(--space-2);
    line-height: 1.45;
  }
  .disclaimer :global(svg) {
    flex-shrink: 0;
    margin-top: 2px;
  }
</style>
