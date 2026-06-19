<script lang="ts">
  import Card from "$lib/components/Card.svelte";
  import Field from "$lib/components/Field.svelte";
  import SegmentedControl from "$lib/components/SegmentedControl.svelte";
  import Stat from "$lib/components/Stat.svelte";
  import Curve from "$lib/components/Curve.svelte";
  import Disclaimer from "$lib/components/Disclaimer.svelte";
  import { simulateSavings, type SavingsResult, type Timing } from "$lib/ipc";
  import { t } from "$lib/i18n";
  import { fmtEUR } from "$lib/format";

  let initial = $state(1000);
  let monthly = $state(200);
  let ratePct = $state(5);
  let years = $state(20);
  let timing = $state<Timing>("end");

  let result = $state<SavingsResult | null>(null);
  let cursorYear = $state<number | undefined>(undefined);
  // Jeton anti-course : seul le dernier calcul lancé peut écrire `result`.
  let runId = 0;

  const timingOptions = $derived([
    { value: "start" as Timing, label: t("savings.timingStart") },
    { value: "end" as Timing, label: t("savings.timingEnd") },
  ]);

  $effect(() => {
    const params = {
      initial,
      monthlyContribution: monthly,
      annualRate: ratePct / 100,
      years,
      timing,
    };
    const myRun = ++runId;
    const id = setTimeout(() => {
      simulateSavings(params).then((r) => {
        if (myRun === runId) result = r;
      });
    }, 40);
    return () => clearTimeout(id);
  });
</script>

<div class="page-head">
  <h1 class="h1">{t("savings.title")}</h1>
  <p class="muted">{t("savings.desc")}</p>
</div>

<div class="cols">
  <Card title={t("common.hypotheses")}>
    <div class="stack">
      <Field
        label={t("savings.initial")}
        bind:value={initial}
        min={0}
        max={200000}
        step={500}
        unit="€"
      />
      <Field
        label={t("savings.monthly")}
        bind:value={monthly}
        min={0}
        max={5000}
        step={50}
        unit={"€" + t("common.perMonth")}
      />
      <Field
        label={t("common.rate")}
        bind:value={ratePct}
        min={0}
        max={20}
        step={0.1}
        unit="%"
      />
      <Field
        label={t("common.duration")}
        bind:value={years}
        min={1}
        max={40}
        step={1}
        unit={t("common.years")}
      />
      <div class="seg-field">
        <span class="small muted">{t("savings.timing")}</span>
        <SegmentedControl
          options={timingOptions}
          bind:value={timing}
          size="sm"
          ariaLabel={t("savings.timing")}
        />
      </div>
    </div>
  </Card>

  <div class="stack-6">
    <Card>
      {#if result}
        <Curve series={result.series} bind:cursorYear valueLabel={t("savings.finalValue")} />
      {/if}
    </Card>

    <Card title={t("common.results")}>
      <div class="statgrid">
        <Stat
          label={t("savings.finalValue")}
          value={result ? fmtEUR(result.finalValue) : "—"}
          tone="accent"
        />
        <Stat
          label={t("savings.totalContributed")}
          value={result ? fmtEUR(result.totalContributed) : "—"}
        />
        <Stat
          label={t("savings.totalInterest")}
          value={result ? fmtEUR(result.totalInterest) : "—"}
          tone="gain"
        />
      </div>
    </Card>

    <Disclaimer />
  </div>
</div>

<style>
  .seg-field {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }
</style>
