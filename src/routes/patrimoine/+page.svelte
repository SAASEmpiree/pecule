<script lang="ts">
  import { Plus, Pencil, Trash2, ArrowRight, Coins } from "@lucide/svelte";
  import Card from "$lib/components/Card.svelte";
  import Stat from "$lib/components/Stat.svelte";
  import CostBar from "$lib/components/CostBar.svelte";
  import Disclaimer from "$lib/components/Disclaimer.svelte";
  import EmptyState from "$lib/components/EmptyState.svelte";
  import SegmentedControl from "$lib/components/SegmentedControl.svelte";
  import {
    listAssets,
    addAsset,
    updateAsset,
    deleteAsset,
    listHoldings,
    type Asset,
    type NewAsset,
    type Holding,
  } from "$lib/ipc";
  import { t } from "$lib/i18n";
  import { fmtEUR } from "$lib/format";

  let assets = $state<Asset[]>([]);
  let holdings = $state<Holding[]>([]);
  let loaded = $state(false);
  let showForm = $state(false);
  let editingId = $state<number | null>(null);
  let saving = $state(false);

  type FormData = { nom: string; type: string; valeur: number };
  const emptyForm = (): FormData => ({ nom: "", type: "compte", valeur: 0 });
  let form = $state<FormData>(emptyForm());

  async function refresh() {
    [assets, holdings] = await Promise.all([listAssets(), listHoldings()]);
    loaded = true;
  }
  $effect(() => {
    refresh();
  });

  const typeOptions = $derived([
    { value: "compte", label: t("networth.typeCompte") },
    { value: "epargne", label: t("networth.typeEpargne") },
    { value: "immobilier", label: t("networth.typeImmobilier") },
    { value: "autre", label: t("networth.typeAutre") },
    { value: "dette", label: t("networth.typeDette") },
  ]);
  function typeLabel(k: string): string {
    const m: Record<string, string> = {
      compte: t("networth.typeCompte"),
      epargne: t("networth.typeEpargne"),
      immobilier: t("networth.typeImmobilier"),
      autre: t("networth.typeAutre"),
      dette: t("networth.typeDette"),
    };
    return m[k] ?? k;
  }

  const portfolioValue = $derived(holdings.reduce((s, h) => s + h.quantite * h.prixActuel, 0));
  const positives = $derived(assets.filter((a) => !a.estDette).reduce((s, a) => s + a.valeur, 0));
  const debtsTotal = $derived(assets.filter((a) => a.estDette).reduce((s, a) => s + a.valeur, 0));
  const assetsTotal = $derived(portfolioValue + positives);
  const netWorth = $derived(assetsTotal - debtsTotal);
  const hasData = $derived(assets.length > 0 || portfolioValue > 0);

  const breakdown = $derived.by(() => {
    const segs: { label: string; value: number; color: string }[] = [];
    if (portfolioValue > 0)
      segs.push({ label: t("networth.portfolioLine"), value: portfolioValue, color: "var(--accent)" });
    const colors: Record<string, string> = {
      compte: "var(--gain)",
      epargne: "var(--soon)",
      immobilier: "var(--muted)",
      autre: "var(--ink)",
    };
    const byType: Record<string, number> = {};
    for (const a of assets) if (!a.estDette) byType[a.type] = (byType[a.type] || 0) + a.valeur;
    for (const [k, v] of Object.entries(byType))
      if (v > 0) segs.push({ label: typeLabel(k), value: v, color: colors[k] ?? "var(--muted)" });
    return segs;
  });

  function startAdd() {
    editingId = null;
    form = emptyForm();
    showForm = true;
  }
  function startEdit(a: Asset) {
    editingId = a.id;
    form = { nom: a.nom, type: a.type, valeur: a.valeur };
    showForm = true;
  }
  const valid = $derived(form.nom.trim() !== "");

  async function submit() {
    if (saving || !valid) return;
    saving = true;
    try {
      const clean: NewAsset = {
        nom: form.nom.trim(),
        type: form.type,
        valeur: Number(form.valeur) || 0,
        estDette: form.type === "dette",
      };
      if (editingId != null) await updateAsset({ id: editingId, ...clean });
      else await addAsset(clean);
      showForm = false;
      await refresh();
    } finally {
      saving = false;
    }
  }
  async function remove(a: Asset) {
    await deleteAsset(a.id);
    if (editingId === a.id) showForm = false;
    await refresh();
  }
</script>

<div class="page-head">
  <h1 class="h1">{t("networth.title")}</h1>
  <p class="small muted">{t("networth.desc")}</p>
</div>

{#if loaded}
  <div class="stack-6">
    {#if showForm}
      <div class="card pad form-card">
        <h2 class="h2">{editingId != null ? t("networth.edit") : t("networth.addAsset")}</h2>
        <div class="form-grid">
          <label class="fld">
            <span class="small muted">{t("networth.name")}</span>
            <input class="in" bind:value={form.nom} placeholder="Livret A" />
          </label>
          <label class="fld">
            <span class="small muted">{t("networth.value")} (€)</span>
            <input class="in mono" type="number" min="0" step="any" bind:value={form.valeur} />
          </label>
          <div class="fld wide">
            <span class="small muted">{t("networth.type")}</span>
            <SegmentedControl
              options={typeOptions}
              bind:value={form.type}
              size="sm"
              ariaLabel={t("networth.type")}
            />
          </div>
        </div>
        <div class="form-actions">
          <button class="btn ghost" type="button" onclick={() => (showForm = false)}>
            {t("networth.cancel")}
          </button>
          <button class="btn cta" type="button" onclick={submit} disabled={!valid || saving}>
            {editingId != null ? t("networth.save") : t("networth.addAsset")}
          </button>
        </div>
      </div>
    {/if}

    {#if !hasData && !showForm}
      <EmptyState
        icon={Coins}
        title={t("networth.emptyTitle")}
        body={t("networth.emptyBody")}
      />
      <div>
        <button class="btn cta" type="button" onclick={startAdd}>
          <Plus size={16} aria-hidden="true" />
          {t("networth.addAsset")}
        </button>
      </div>
    {:else if hasData}
      <Card>
        <div class="statgrid">
          <Stat label={t("networth.total")} value={fmtEUR(netWorth)} tone={netWorth >= 0 ? "accent" : "loss"} />
          <Stat label={t("networth.assetsTotal")} value={fmtEUR(assetsTotal)} tone="gain" />
          <Stat
            label={t("networth.debtsTotal")}
            value={debtsTotal > 0 ? `− ${fmtEUR(debtsTotal)}` : fmtEUR(0)}
            tone={debtsTotal > 0 ? "loss" : "neutral"}
          />
        </div>
        {#if breakdown.length}
          <div class="alloc">
            <span class="small muted">{t("networth.breakdown")}</span>
            <CostBar segments={breakdown} format={fmtEUR} ariaLabel={t("networth.breakdown")} />
          </div>
        {/if}
      </Card>

      <Card title={t("networth.title")}>
        {#snippet actions()}
          <button class="btn cta sm" type="button" onclick={startAdd}>
            <Plus size={14} aria-hidden="true" />
            {t("networth.addAsset")}
          </button>
        {/snippet}
        <div class="table-scroll">
          <table class="nw">
            <thead>
              <tr>
                <th>{t("networth.name")}</th>
                <th>{t("networth.type")}</th>
                <th class="num">{t("networth.value")}</th>
                <th aria-label="actions"></th>
              </tr>
            </thead>
            <tbody>
              {#if portfolioValue > 0}
                <tr class="pf-row">
                  <td><a class="nm link" href="/portefeuille">{t("networth.portfolioLine")} <ArrowRight size={13} aria-hidden="true" /></a></td>
                  <td><span class="tag accent-tag">{t("nav.portfolio")}</span></td>
                  <td class="num mono strong">{fmtEUR(portfolioValue)}</td>
                  <td></td>
                </tr>
              {/if}
              {#each assets as a (a.id)}
                <tr>
                  <td><div class="nm">{a.nom}</div></td>
                  <td><span class="tag">{typeLabel(a.type)}</span></td>
                  <td class="num mono strong" class:loss={a.estDette}>
                    {a.estDette ? `− ${fmtEUR(a.valeur)}` : fmtEUR(a.valeur)}
                  </td>
                  <td class="row-actions">
                    <button class="icon" type="button" onclick={() => startEdit(a)} aria-label={t("networth.edit")}>
                      <Pencil size={15} aria-hidden="true" />
                    </button>
                    <button class="icon" type="button" onclick={() => remove(a)} aria-label={t("networth.delete")}>
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
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
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
  table.nw {
    width: 100%;
    border-collapse: collapse;
    font-size: var(--fs-small);
  }
  .nw th {
    text-align: left;
    font-weight: 500;
    color: var(--muted);
    padding: 8px 10px;
    border-bottom: 1px solid var(--line);
    white-space: nowrap;
  }
  .nw td {
    padding: 10px;
    border-bottom: 1px solid var(--line);
    vertical-align: middle;
  }
  .nw tbody tr:last-child td {
    border-bottom: 0;
  }
  .nw .num {
    text-align: right;
    white-space: nowrap;
  }
  .nw .strong {
    color: var(--ink);
    font-weight: 500;
  }
  .nm {
    color: var(--ink);
    font-weight: 500;
  }
  .nm.link {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    color: var(--accent);
  }
  .tag {
    display: inline-block;
    padding: 2px 9px;
    border-radius: var(--radius-pill);
    background: var(--surface-2);
    border: 1px solid var(--line);
    color: var(--muted);
    font-size: 11px;
    white-space: nowrap;
  }
  .accent-tag {
    background: var(--accent-soft);
    color: var(--accent);
    border-color: transparent;
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
