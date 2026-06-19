<script lang="ts">
  import { Wallet, CalendarClock, ArrowRight } from "@lucide/svelte";
  import Card from "$lib/components/Card.svelte";
  import Curve from "$lib/components/Curve.svelte";
  import Field from "$lib/components/Field.svelte";
  import Stat from "$lib/components/Stat.svelte";
  import Disclaimer from "$lib/components/Disclaimer.svelte";
  import { simulateSavings, type SavingsResult } from "$lib/ipc";
  import { t } from "$lib/i18n";
  import { fmtEUR, fmtEUR0, fmtPct } from "$lib/format";

  // Projection d'exemple (aucune donnée personnelle en v0.1) : elle fait vivre
  // « la courbe » et relie la pédagogie aux futurs piliers.
  const SAMPLE_RATE = 0.05;
  const SAMPLE_YEARS = 30;
  let monthly = $state(200);

  let result = $state<SavingsResult | null>(null);
  let cursorYear = $state<number | undefined>(undefined);

  $effect(() => {
    const params = {
      initial: 0,
      monthlyContribution: monthly,
      annualRate: SAMPLE_RATE,
      years: SAMPLE_YEARS,
      timing: "end" as const,
    };
    const id = setTimeout(() => {
      simulateSavings(params).then((r) => (result = r));
    }, 40);
    return () => clearTimeout(id);
  });

  const sampleNote = $derived(
    t("dashboard.sampleNote", {
      monthly: fmtEUR0(monthly),
      per: t("common.perMonth"),
      rate: fmtPct(SAMPLE_RATE, 0),
    }),
  );

  const pillars = $derived([
    { href: "/portefeuille", icon: Wallet, label: t("nav.portfolio"), badge: "v0.2" },
    { href: "/abonnements", icon: CalendarClock, label: t("nav.subscriptions"), badge: "v0.3" },
  ]);
</script>

<div class="page-head">
  <h1 class="h1">{t("nav.dashboard")}</h1>
  <p class="muted">{t("app.tagline")}</p>
</div>

<div class="stack-6">
  <Card title={t("dashboard.curveTitle")} subtitle={sampleNote}>
    <div class="hero">
      <div class="control">
        <Field
          label={t("savings.monthly")}
          bind:value={monthly}
          min={0}
          max={2000}
          step={50}
          unit={"€" + t("common.perMonth")}
        />
        <a class="cta" href="/simulateurs/epargne">
          {t("dashboard.exploreSimulators")}
          <ArrowRight size={15} aria-hidden="true" />
        </a>
      </div>

      {#if result}
        <Curve series={result.series} bind:cursorYear valueLabel={t("dashboard.projectedValue")} />

        <div class="statrow">
          <Stat
            label={t("savings.finalValue")}
            value={fmtEUR(result.finalValue)}
            sub={t("common.inYears", { n: SAMPLE_YEARS })}
            tone="accent"
          />
          <Stat label={t("savings.totalContributed")} value={fmtEUR(result.totalContributed)} />
          <Stat
            label={t("savings.totalInterest")}
            value={fmtEUR(result.totalInterest)}
            tone="gain"
          />
        </div>
      {/if}
    </div>
  </Card>

  <div class="pillars">
    {#each pillars as p (p.href)}
      {@const Icon = p.icon}
      <a class="pillar card pad" href={p.href}>
        <div class="ic"><Icon size={20} aria-hidden="true" /></div>
        <span class="lbl">{p.label}</span>
        <span class="badge">{p.badge}</span>
      </a>
    {/each}
  </div>

  <Disclaimer />
</div>

<style>
  .hero {
    display: flex;
    flex-direction: column;
    gap: var(--space-6);
  }
  .control {
    display: flex;
    align-items: flex-end;
    justify-content: space-between;
    gap: var(--space-4);
    flex-wrap: wrap;
  }
  .control :global(.field) {
    flex: 1;
    min-width: 220px;
  }
  .cta {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 10px 16px;
    border-radius: var(--radius-sm);
    background: var(--accent);
    color: var(--accent-ink);
    font-weight: 600;
    white-space: nowrap;
    transition: filter var(--dur-fast) var(--ease);
  }
  .cta:hover {
    filter: brightness(1.08);
  }
  .statrow {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: var(--space-4);
    padding-top: var(--space-4);
    border-top: 1px solid var(--line);
  }
  @media (max-width: 640px) {
    .statrow {
      grid-template-columns: 1fr;
      gap: var(--space-5);
    }
  }
  .pillars {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--space-4);
  }
  @media (max-width: 640px) {
    .pillars {
      grid-template-columns: 1fr;
    }
  }
  .pillar {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    color: var(--ink);
    transition: border-color var(--dur-fast) var(--ease);
  }
  .pillar:hover {
    border-color: var(--accent);
  }
  .ic {
    width: 38px;
    height: 38px;
    display: grid;
    place-items: center;
    border-radius: var(--radius-sm);
    background: var(--accent-soft);
    color: var(--accent);
  }
  .lbl {
    font-weight: 500;
  }
  .badge {
    margin-left: auto;
    background: var(--soon-soft);
    color: var(--soon);
    padding: 2px 9px;
    border-radius: var(--radius-pill);
    font-size: 11px;
    font-weight: 600;
  }
</style>
