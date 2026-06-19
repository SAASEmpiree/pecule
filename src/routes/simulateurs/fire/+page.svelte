<script lang="ts">
  import { Flame } from "@lucide/svelte";
  import EmptyState from "$lib/components/EmptyState.svelte";
  import { t } from "$lib/i18n";
</script>

<div class="page-head">
  <h1 class="h1">{t("fire.title")}</h1>
</div>

<EmptyState icon={Flame} title={t("fire.title")} body={t("fire.body")} badge="v2" />
