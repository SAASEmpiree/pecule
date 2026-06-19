<script lang="ts">
  import { Plus, Pencil, Trash2, Receipt } from "@lucide/svelte";
  import Card from "$lib/components/Card.svelte";
  import Stat from "$lib/components/Stat.svelte";
  import CostBar from "$lib/components/CostBar.svelte";
  import Disclaimer from "$lib/components/Disclaimer.svelte";
  import EmptyState from "$lib/components/EmptyState.svelte";
  import SegmentedControl from "$lib/components/SegmentedControl.svelte";
  import {
    listExpenses,
    addExpense,
    updateExpense,
    deleteExpense,
    type Expense,
    type NewExpense,
  } from "$lib/ipc";
  import { t } from "$lib/i18n";
  import { fmtEUR } from "$lib/format";

  let expenses = $state<Expense[]>([]);
  let loaded = $state(false);
  let showForm = $state(false);
  let editingId = $state<number | null>(null);
  let saving = $state(false);

  const todayIso = () => new Date().toISOString().slice(0, 10);
  const monthPrefix = new Date().toISOString().slice(0, 7);

  type FormData = { date: string; montant: number; categorie: string; kind: string; note: string };
  const emptyForm = (): FormData => ({
    date: todayIso(),
    montant: 0,
    categorie: "",
    kind: "depense",
    note: "",
  });
  let form = $state<FormData>(emptyForm());

  async function refresh() {
    expenses = await listExpenses();
    loaded = true;
  }
  $effect(() => {
    refresh();
  });

  const kindOptions = $derived([
    { value: "depense", label: t("budget.expense") },
    { value: "revenu", label: t("budget.income") },
  ]);

  const monthExpenses = $derived(expenses.filter((e) => e.date.startsWith(monthPrefix)));
  const revenus = $derived(
    monthExpenses.filter((e) => e.kind === "revenu").reduce((s, e) => s + e.montant, 0),
  );
  const depenses = $derived(
    monthExpenses.filter((e) => e.kind === "depense").reduce((s, e) => s + e.montant, 0),
  );
  const solde = $derived(revenus - depenses);

  const byCategory = $derived.by(() => {
    const colors = ["var(--accent)", "var(--gain)", "var(--soon)", "var(--loss)", "var(--muted)"];
    const sums: Record<string, number> = {};
    for (const e of monthExpenses)
      if (e.kind === "depense") sums[e.categorie || "—"] = (sums[e.categorie || "—"] || 0) + e.montant;
    return Object.entries(sums)
      .filter(([, v]) => v > 0)
      .sort((a, b) => b[1] - a[1])
      .map(([k, v], i) => ({ label: k, value: v, color: colors[i % colors.length] }));
  });

  function startAdd() {
    editingId = null;
    form = emptyForm();
    showForm = true;
  }
  function startEdit(e: Expense) {
    editingId = e.id;
    form = {
      date: e.date,
      montant: e.montant,
      categorie: e.categorie,
      kind: e.kind,
      note: e.note ?? "",
    };
    showForm = true;
  }
  const valid = $derived(form.date !== "" && (Number(form.montant) || 0) > 0);

  async function submit() {
    if (saving || !valid) return;
    saving = true;
    try {
      const clean: NewExpense = {
        date: form.date,
        montant: Number(form.montant) || 0,
        categorie: form.categorie.trim() || (form.kind === "revenu" ? t("budget.income") : t("budget.expense")),
        kind: form.kind,
        note: form.note.trim() || null,
      };
      if (editingId != null) await updateExpense({ id: editingId, ...clean });
      else await addExpense(clean);
      showForm = false;
      await refresh();
    } finally {
      saving = false;
    }
  }
  async function remove(e: Expense) {
    await deleteExpense(e.id);
    if (editingId === e.id) showForm = false;
    await refresh();
  }
</script>

<div class="page-head">
  <h1 class="h1">{t("budget.title")}</h1>
  <p class="small muted">{t("budget.desc")}</p>
</div>

{#if loaded}
  <div class="stack-6">
    {#if showForm}
      <div class="card pad form-card">
        <h2 class="h2">{editingId != null ? t("budget.edit") : t("budget.addOp")}</h2>
        <div class="form-grid">
          <div class="fld">
            <span class="small muted">{t("budget.kind")}</span>
            <SegmentedControl options={kindOptions} bind:value={form.kind} size="sm" ariaLabel={t("budget.kind")} />
          </div>
          <label class="fld">
            <span class="small muted">{t("budget.amount")} (€)</span>
            <input class="in mono" type="number" min="0" step="any" bind:value={form.montant} />
          </label>
          <label class="fld">
            <span class="small muted">{t("budget.category")}</span>
            <input class="in" bind:value={form.categorie} placeholder="Courses" />
          </label>
          <label class="fld">
            <span class="small muted">{t("budget.date")}</span>
            <input class="in mono" type="date" bind:value={form.date} />
          </label>
          <label class="fld wide">
            <span class="small muted">{t("budget.note")}</span>
            <input class="in" bind:value={form.note} />
          </label>
        </div>
        <div class="form-actions">
          <button class="btn ghost" type="button" onclick={() => (showForm = false)}>{t("budget.cancel")}</button>
          <button class="btn cta" type="button" onclick={submit} disabled={!valid || saving}>
            {editingId != null ? t("budget.save") : t("budget.addOp")}
          </button>
        </div>
      </div>
    {/if}

    {#if expenses.length === 0 && !showForm}
      <EmptyState icon={Receipt} title={t("budget.emptyTitle")} body={t("budget.emptyBody")} />
      <div>
        <button class="btn cta" type="button" onclick={startAdd}>
          <Plus size={16} aria-hidden="true" />
          {t("budget.addOp")}
        </button>
      </div>
    {:else if expenses.length > 0}
      <Card title={t("budget.thisMonth")}>
        <div class="statgrid">
          <Stat label={t("budget.revenus")} value={fmtEUR(revenus)} tone="gain" />
          <Stat label={t("budget.depenses")} value={fmtEUR(depenses)} tone="loss" />
          <Stat label={t("budget.solde")} value={fmtEUR(solde)} tone={solde >= 0 ? "accent" : "loss"} />
        </div>
        {#if byCategory.length}
          <div class="alloc">
            <span class="small muted">{t("budget.byCategory")}</span>
            <CostBar segments={byCategory} format={fmtEUR} ariaLabel={t("budget.byCategory")} />
          </div>
        {/if}
      </Card>

      <Card title={t("budget.title")}>
        {#snippet actions()}
          <button class="btn cta sm" type="button" onclick={startAdd}>
            <Plus size={14} aria-hidden="true" />
            {t("budget.addOp")}
          </button>
        {/snippet}
        <div class="table-scroll">
          <table class="bud">
            <thead>
              <tr>
                <th>{t("budget.date")}</th>
                <th>{t("budget.category")}</th>
                <th class="num">{t("budget.amount")}</th>
                <th aria-label="actions"></th>
              </tr>
            </thead>
            <tbody>
              {#each expenses as e (e.id)}
                <tr>
                  <td class="mono muted">{e.date}</td>
                  <td>
                    <div class="nm">{e.categorie}</div>
                    {#if e.note}<div class="small muted">{e.note}</div>{/if}
                  </td>
                  <td class="num mono strong" class:gain={e.kind === "revenu"} class:loss={e.kind === "depense"}>
                    {e.kind === "revenu" ? "+ " : "− "}{fmtEUR(e.montant)}
                  </td>
                  <td class="row-actions">
                    <button class="icon" type="button" onclick={() => startEdit(e)} aria-label={t("budget.edit")}>
                      <Pencil size={15} aria-hidden="true" />
                    </button>
                    <button class="icon" type="button" onclick={() => remove(e)} aria-label={t("budget.delete")}>
                      <Trash2 size={15} aria-hidden="true" />
                    </button>
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      </Card>

      <Disclaimer />
    {/if}
  </div>
{/if}

<style>
  .btn {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 10px 16px;
    border-radius: var(--radius-sm);
    border: 1px solid transparent;
    font-weight: 600;
    font-size: var(--fs-body);
    transition: filter var(--dur-fast) var(--ease), background var(--dur-fast) var(--ease);
  }
  .btn.sm {
    padding: 7px 12px;
    font-size: var(--fs-small);
  }
  .btn.cta {
    background: var(--accent);
    color: var(--accent-ink);
  }
  .btn.cta:hover {
    filter: brightness(1.08);
  }
  .btn.cta:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  .btn.ghost {
    background: var(--surface-2);
    border-color: var(--line);
    color: var(--ink);
  }
  .alloc {
    margin-top: var(--space-6);
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }
  .form-card {
    display: flex;
    flex-direction: column;
    gap: var(--space-5);
    border-color: var(--accent);
  }
  .form-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(190px, 1fr));
    gap: var(--space-4);
  }
  .fld {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }
  .fld.wide {
    grid-column: 1 / -1;
  }
  .in {
    background: var(--surface-2);
    border: 1px solid var(--line);
    border-radius: var(--radius-sm);
    padding: 10px var(--space-3);
    color: var(--ink);
    font-size: var(--fs-body);
    font-weight: 500;
    outline: none;
  }
  .in:focus {
    border-color: var(--accent);
  }
  .form-actions {
    display: flex;
    justify-content: flex-end;
    gap: var(--space-3);
  }
  .table-scroll {
    overflow-x: auto;
  }
  table.bud {
    width: 100%;
    border-collapse: collapse;
    font-size: var(--fs-small);
  }
  .bud th {
    text-align: left;
    font-weight: 500;
    color: var(--muted);
    padding: 8px 10px;
    border-bottom: 1px solid var(--line);
    white-space: nowrap;
  }
  .bud td {
    padding: 10px;
    border-bottom: 1px solid var(--line);
    vertical-align: middle;
  }
  .bud tbody tr:last-child td {
    border-bottom: 0;
  }
  .bud .num {
    text-align: right;
    white-space: nowrap;
  }
  .bud .strong {
    font-weight: 500;
  }
  .nm {
    color: var(--ink);
    font-weight: 500;
  }
  .row-actions {
    display: flex;
    gap: 4px;
    justify-content: flex-end;
  }
  .icon {
    display: inline-grid;
    place-items: center;
    width: 30px;
    height: 30px;
    border-radius: var(--radius-sm);
    border: 1px solid transparent;
    background: transparent;
    color: var(--muted);
    transition: background var(--dur-fast) var(--ease), color var(--dur-fast) var(--ease);
  }
  .icon:hover {
    background: var(--surface-2);
    color: var(--ink);
  }
</style>
