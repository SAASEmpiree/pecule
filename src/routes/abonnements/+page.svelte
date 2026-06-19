<script lang="ts">
  import { CalendarClock } from "@lucide/svelte";
  import EmptyState from "$lib/components/EmptyState.svelte";
  import { t } from "$lib/i18n";
</script>

<div class="page-head">
  <h1 class="h1">{t("subscriptions.title")}</h1>
</div>

<EmptyState
  icon={CalendarClock}
  title={t("subscriptions.emptyTitle")}
  body={t("subscriptions.emptyBody")}
  badge="v0.3"
/>
