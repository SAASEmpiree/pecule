<script lang="ts">
  import Card from "$lib/components/Card.svelte";
  import Field from "$lib/components/Field.svelte";
  import Stat from "$lib/components/Stat.svelte";
  import Disclaimer from "$lib/components/Disclaimer.svelte";
  import { t } from "$lib/i18n";
  import { fmtEUR } from "$lib/format";

  let gain = $state(10000);

  const base = $derived(Math.max(gain, 0));
  const ir = $derived(base * 0.128);
  const ps = $derived(base * 0.172);
  const totalTax = $derived(ir + ps);
  const net = $derived(base - totalTax);

  const envelopes = $derived([
    { name: t("tax.pea"), desc: t("tax.peaDesc") },
    { name: t("tax.cto"), desc: t("tax.ctoDesc") },
    { name: t("tax.av"), desc: t("tax.avDesc") },
    { name: t("tax.per"), desc: t("tax.perDesc") },
  ]);
</script>

<div class="page-head">
  <h1 class="h1">{t("tax.title")}</h1>
  <p class="muted">{t("tax.desc")}</p>
</div>

<div class="stack-6">
  <Card title={t("tax.pfuTitle")}>
    <div class="cols2">
      <Field label={t("tax.gain")} bind:value={gain} min={0} max={1000000} step={500} unit="€" />
      <div class="statgrid">
        <Stat label={t("tax.ir")} value={fmtEUR(ir)} tone="loss" />
        <Stat label={t("tax.ps")} value={fmtEUR(ps)} tone="loss" />
        <Stat label={t("tax.totalTax")} value={fmtEUR(totalTax)} tone="loss" />
        <Stat label={t("tax.net")} value={fmtEUR(net)} tone="accent" />
      </div>
    </div>
  </Card>

  <Card title={t("tax.envelopes")}>
    <div class="env-grid">
      {#each envelopes as e (e.name)}
        <div class="env">
          <div class="env-name">{e.name}</div>
          <p class="small muted">{e.desc}</p>
        </div>
      {/each}
    </div>
  </Card>

  <Disclaimer long />
</div>

<style>
  .cols2 {
    display: grid;
    grid-template-columns: 300px 1fr;
    gap: var(--space-8);
    align-items: center;
  }
  @media (max-width: 780px) {
    .cols2 {
      grid-template-columns: 1fr;
      gap: var(--space-6);
    }
  }
  .env-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
    gap: var(--space-4);
  }
  .env {
    background: var(--surface-2);
    border: 1px solid var(--line);
    border-radius: var(--radius-md);
    padding: var(--space-4);
  }
  .env-name {
    font-weight: 600;
    color: var(--ink);
    margin-bottom: 4px;
  }
  .env p {
    line-height: 1.5;
  }
</style>
