<script lang="ts">
  import Card from "$lib/components/Card.svelte";
  import Field from "$lib/components/Field.svelte";
  import SegmentedControl from "$lib/components/SegmentedControl.svelte";
  import Stat from "$lib/components/Stat.svelte";
  import Curve from "$lib/components/Curve.svelte";
  import Disclaimer from "$lib/components/Disclaimer.svelte";
  import { simulateCompound, type CompoundResult } from "$lib/ipc";
  import { t } from "$lib/i18n";
  import { fmtEUR } from "$lib/format";

  let principal = $state(10000);
  let ratePct = $state(5);
  let years = $state(20);
  let freq = $state(12);

  let result = $state<CompoundResult | null>(null);
  let cursorYear = $state<number | undefined>(undefined);

  const freqOptions = $derived([
    { value: 12, label: t("compound.freqMonthly") },
    { value: 4, label: t("compound.freqQuarterly") },
    { value: 1, label: t("compound.freqAnnual") },
  ]);

  $effect(() => {
    const params = {
      principal,
      annualRate: ratePct / 100,
      years,
      compoundsPerYear: freq,
    };
    const id = setTimeout(() => {
      simulateCompound(params).then((r) => (result = r));
    }, 40);
    return () => clearTimeout(id);
  });
</script>

<div class="page-head">
  <h1 class="h1">{t("compound.title")}</h1>
  <p class="muted">{t("compound.desc")}</p>
</div>

<div class="cols">
  <Card title={t("common.hypotheses")}>
    <div class="stack">
      <Field
        label={t("compound.principal")}
        bind:value={principal}
        min={0}
        max={500000}
        step={500}
        unit="€"
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
        <span class="small muted">{t("compound.frequency")}</span>
        <SegmentedControl options={freqOptions} bind:value={freq} size="sm" ariaLabel={t("compound.frequency")} />
      </div>
    </div>
  </Card>

  <div class="stack-6">
    <Card>
      {#if result}
        <Curve
          series={result.series}
          bind:cursorYear
          showContributed={false}
          valueLabel={t("compound.finalValue")}
        />
      {/if}
    </Card>

    <Card title={t("common.results")}>
      <div class="statgrid">
        <Stat
          label={t("compound.finalValue")}
          value={result ? fmtEUR(result.finalValue) : "—"}
          tone="accent"
        />
        <Stat
          label={t("compound.totalInterest")}
          value={result ? fmtEUR(result.totalInterest) : "—"}
          tone="gain"
        />
        <Stat
          label={t("common.contributed")}
          value={result ? fmtEUR(result.totalContributed) : "—"}
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
