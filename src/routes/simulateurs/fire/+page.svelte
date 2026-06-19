<script lang="ts">
  import Card from "$lib/components/Card.svelte";
  import Field from "$lib/components/Field.svelte";
  import Stat from "$lib/components/Stat.svelte";
  import Curve from "$lib/components/Curve.svelte";
  import Disclaimer from "$lib/components/Disclaimer.svelte";
  import { simulateFire, type FireResult } from "$lib/ipc";
  import { t } from "$lib/i18n";
  import { fmtEUR, fmtNum } from "$lib/format";

  let initialCapital = $state(20000);
  let monthly = $state(800);
  let ratePct = $state(5);
  let annualExpenses = $state(24000);
  let withdrawalPct = $state(4);

  let result = $state<FireResult | null>(null);
  let cursorYear = $state<number | undefined>(undefined);
  // Jeton anti-course : seul le dernier calcul lancé peut écrire `result`.
  let runId = 0;

  $effect(() => {
    const params = {
      initialCapital,
      monthlyContribution: monthly,
      annualRate: ratePct / 100,
      annualExpenses,
      withdrawalRate: withdrawalPct / 100,
    };
    const myRun = ++runId;
    const id = setTimeout(() => {
      simulateFire(params).then((r) => {
        if (myRun === runId) result = r;
      });
    }, 40);
    return () => clearTimeout(id);
  });

  const timeToFi = $derived(
    result && result.yearsToFi != null
      ? `${fmtNum(result.yearsToFi, 1)} ${t("common.years")}`
      : "—",
  );
</script>

<div class="page-head">
  <h1 class="h1">{t("fire.title")}</h1>
  <p class="muted">{t("fire.desc")}</p>
</div>

<div class="cols">
  <Card title={t("common.hypotheses")}>
    <div class="stack">
      <Field
        label={t("fire.initialCapital")}
        bind:value={initialCapital}
        min={0}
        max={1000000}
        step={1000}
        unit="€"
      />
      <Field
        label={t("fire.monthly")}
        bind:value={monthly}
        min={0}
        max={10000}
        step={50}
        unit={"€" + t("common.perMonth")}
      />
      <Field label={t("common.rate")} bind:value={ratePct} min={0} max={15} step={0.1} unit="%" />
      <Field
        label={t("fire.annualExpenses")}
        bind:value={annualExpenses}
        min={0}
        max={200000}
        step={1000}
        unit="€"
      />
      <Field
        label={t("fire.withdrawalRate")}
        bind:value={withdrawalPct}
        min={1}
        max={10}
        step={0.1}
        unit="%"
      />
    </div>
  </Card>

  <div class="stack-6">
    <Card title={t("common.results")}>
      <div class="statgrid">
        <Stat label={t("fire.fiNumber")} value={result ? fmtEUR(result.fiNumber) : "—"} tone="accent" />
        <Stat
          label={t("fire.timeToFi")}
          value={timeToFi}
          tone={result?.targetReached ? "gain" : "neutral"}
        />
      </div>
      {#if result && !result.targetReached}
        <p class="small muted note">{t("fire.notReached")}</p>
      {/if}
    </Card>

    <Card>
      {#if result}
        <Curve series={result.series} bind:cursorYear valueLabel={t("fire.projectedCapital")} />
      {/if}
    </Card>

    <Disclaimer />
  </div>
</div>

<style>
  .note {
    margin-top: var(--space-4);
    line-height: 1.5;
  }
</style>
