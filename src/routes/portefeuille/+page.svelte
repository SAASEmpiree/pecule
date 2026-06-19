<script lang="ts">
  import { Wallet } from "@lucide/svelte";
  import EmptyState from "$lib/components/EmptyState.svelte";
  import { t } from "$lib/i18n";
</script>

<div class="page-head">
  <h1 class="h1">{t("portfolio.title")}</h1>
</div>

<EmptyState
  icon={Wallet}
  title={t("portfolio.emptyTitle")}
  body={t("portfolio.emptyBody")}
  badge="v0.2"
/>
