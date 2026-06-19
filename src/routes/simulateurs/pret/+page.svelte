<script lang="ts">
  import Card from "$lib/components/Card.svelte";
  import Field from "$lib/components/Field.svelte";
  import Stat from "$lib/components/Stat.svelte";
  import CostBar from "$lib/components/CostBar.svelte";
  import Disclaimer from "$lib/components/Disclaimer.svelte";
  import { simulateLoan, type LoanResult } from "$lib/ipc";
  import { t } from "$lib/i18n";
  import { fmtEUR } from "$lib/format";

  let amount = $state(200000);
  let ratePct = $state(3.5);
  let termYears = $state(20);
  let insurancePct = $state(0.34);

  let result = $state<LoanResult | null>(null);
  let showMonthly = $state(false);
  // Jeton anti-course : seul le dernier calcul lancé peut écrire `result`.
  let runId = 0;

  $effect(() => {
    const params = {
      principal: amount,
      annualRate: ratePct / 100,
      termMonths: Math.round(termYears * 12),
      insuranceAnnualRate: insurancePct / 100,
    };
    const myRun = ++runId;
    const id = setTimeout(() => {
      simulateLoan(params).then((r) => {
        if (myRun === runId) result = r;
      });
    }, 40);
    return () => clearTimeout(id);
  });

  const segments = $derived(
    result
      ? [
          { label: t("loan.principalPart"), value: amount, color: "var(--accent)" },
          { label: t("loan.totalInterest"), value: result.totalInterest, color: "var(--loss)" },
          { label: t("loan.totalInsurance"), value: result.totalInsurance, color: "var(--soon)" },
        ]
      : [],
  );

  const yearly = $derived.by(() => {
    if (!result) return [];
    const sched = result.schedule;
    const rows: {
      year: number;
      principalSum: number;
      interestSum: number;
      insSum: number;
      balance: number;
    }[] = [];
    for (let y = 0; y * 12 < sched.length; y++) {
      const chunk = sched.slice(y * 12, y * 12 + 12);
      rows.push({
        year: y + 1,
        principalSum: chunk.reduce((s, r) => s + r.principalPart, 0),
        interestSum: chunk.reduce((s, r) => s + r.interestPart, 0),
        insSum: chunk.reduce((s, r) => s + r.insurance, 0),
        balance: chunk[chunk.length - 1].balance,
      });
    }
    return rows;
  });

  const hasInsurance = $derived(insurancePct > 0);
</script>

<div class="page-head">
  <h1 class="h1">{t("loan.title")}</h1>
  <p class="muted">{t("loan.desc")}</p>
</div>

<div class="cols">
  <Card title={t("common.hypotheses")}>
    <div class="stack">
      <Field
        label={t("loan.amount")}
        bind:value={amount}
        min={1000}
        max={1000000}
        step={5000}
        unit="€"
      />
      <Field
        label={t("common.rate")}
        bind:value={ratePct}
        min={0}
        max={10}
        step={0.05}
        unit="%"
      />
      <Field
        label={t("loan.term")}
        bind:value={termYears}
        min={1}
        max={30}
        step={1}
        unit={t("common.years")}
      />
      <Field
        label={t("loan.insurance")}
        bind:value={insurancePct}
        min={0}
        max={1}
        step={0.01}
        unit="%"
      />
    </div>
  </Card>

  <div class="stack-6">
    <Card title={t("common.results")}>
      <div class="statgrid">
        <Stat
          label={hasInsurance ? t("loan.monthlyTotal") : t("loan.monthlyPayment")}
          value={result ? fmtEUR(result.monthlyTotal) : "—"}
          tone="accent"
        />
        {#if hasInsurance}
          <Stat
            label={t("loan.monthlyPayment")}
            value={result ? fmtEUR(result.monthlyPayment) : "—"}
          />
          <Stat
            label={t("loan.monthlyInsurance")}
            value={result ? fmtEUR(result.monthlyInsurance) : "—"}
          />
        {/if}
        <Stat
          label={t("loan.totalInterest")}
          value={result ? fmtEUR(result.totalInterest) : "—"}
          tone="loss"
        />
        {#if hasInsurance}
          <Stat
            label={t("loan.totalInsurance")}
            value={result ? fmtEUR(result.totalInsurance) : "—"}
          />
        {/if}
        <Stat
          label={t("loan.totalCost")}
          value={result ? fmtEUR(result.totalCost) : "—"}
        />
      </div>

      {#if result}
        <div class="bar-wrap">
          <CostBar {segments} format={fmtEUR} ariaLabel={t("loan.totalCost")} />
        </div>
      {/if}
    </Card>

    <Card title={t("loan.schedule")}>
      {#snippet actions()}
        <button class="link-btn small" type="button" onclick={() => (showMonthly = !showMonthly)}>
          {showMonthly ? t("loan.hideSchedule") : t("loan.showSchedule")}
        </button>
      {/snippet}

      <div class="table-scroll" class:tall={showMonthly}>
        <table class="amort">
          <thead>
            <tr>
              <th>{showMonthly ? t("loan.month") : t("loan.yearCol")}</th>
              <th class="num">{t("loan.principalPart")}</th>
              <th class="num">{t("loan.interestPart")}</th>
              {#if hasInsurance}<th class="num">{t("loan.insuranceCol")}</th>{/if}
              <th class="num">{t("loan.balance")}</th>
            </tr>
          </thead>
          <tbody>
            {#if showMonthly && result}
              {#each result.schedule as row (row.month)}
                <tr>
                  <td class="mono">{row.month}</td>
                  <td class="num mono">{fmtEUR(row.principalPart)}</td>
                  <td class="num mono loss">{fmtEUR(row.interestPart)}</td>
                  {#if hasInsurance}<td class="num mono">{fmtEUR(row.insurance)}</td>{/if}
                  <td class="num mono muted">{fmtEUR(row.balance)}</td>
                </tr>
              {/each}
            {:else}
              {#each yearly as row (row.year)}
                <tr>
                  <td class="mono">{t("loan.yearN", { n: row.year })}</td>
                  <td class="num mono">{fmtEUR(row.principalSum)}</td>
                  <td class="num mono loss">{fmtEUR(row.interestSum)}</td>
                  {#if hasInsurance}<td class="num mono">{fmtEUR(row.insSum)}</td>{/if}
                  <td class="num mono muted">{fmtEUR(row.balance)}</td>
                </tr>
              {/each}
            {/if}
          </tbody>
        </table>
      </div>
    </Card>

    <Disclaimer />
  </div>
</div>

<style>
  .bar-wrap {
    margin-top: var(--space-6);
  }
  .link-btn {
    border: 0;
    background: transparent;
    color: var(--accent);
    font-weight: 500;
    padding: 0;
  }
  .link-btn:hover {
    text-decoration: underline;
  }
  .table-scroll {
    overflow-x: auto;
  }
  .table-scroll.tall {
    max-height: 360px;
    overflow-y: auto;
  }
  table.amort {
    width: 100%;
    border-collapse: collapse;
    font-size: var(--fs-small);
  }
  .amort th {
    text-align: left;
    font-weight: 500;
    color: var(--muted);
    padding: 8px 10px;
    border-bottom: 1px solid var(--line);
    position: sticky;
    top: 0;
    background: var(--surface);
  }
  .amort td {
    padding: 7px 10px;
    border-bottom: 1px solid var(--line);
  }
  .amort tbody tr:last-child td {
    border-bottom: 0;
  }
  .amort .num {
    text-align: right;
  }
</style>
