<script lang="ts">
  import { Plus, Pencil, Trash2, Download, Wallet } from "@lucide/svelte";
  import Card from "$lib/components/Card.svelte";
  import Stat from "$lib/components/Stat.svelte";
  import CostBar from "$lib/components/CostBar.svelte";
  import Disclaimer from "$lib/components/Disclaimer.svelte";
  import EmptyState from "$lib/components/EmptyState.svelte";
  import SegmentedControl from "$lib/components/SegmentedControl.svelte";
  import {
    listHoldings,
    addHolding,
    updateHolding,
    deleteHolding,
    type Holding,
    type NewHolding,
  } from "$lib/ipc";
  import { t } from "$lib/i18n";
  import { fmtEUR, fmtSignedEUR, fmtSignedPct, fmtNum } from "$lib/format";

  let holdings = $state<Holding[]>([]);
  let loaded = $state(false);

  let showForm = $state(false);
  let editingId = $state<number | null>(null);
  let saving = $state(false);

  const emptyForm = (): NewHolding => ({
    nom: "",
    symbole: "",
    type: "action",
    quantite: 1,
    pru: 0,
    prixActuel: 0,
    devise: "EUR",
  });
  let form = $state<NewHolding>(emptyForm());

  async function refresh() {
    holdings = await listHoldings();
    loaded = true;
  }
  $effect(() => {
    refresh();
  });

  const typeOptions = $derived([
    { value: "action", label: t("portfolio.typeAction") },
    { value: "etf", label: t("portfolio.typeEtf") },
    { value: "crypto", label: t("portfolio.typeCrypto") },
    { value: "autre", label: t("portfolio.typeOther") },
  ]);

  function typeLabel(k: string): string {
    const m: Record<string, string> = {
      action: t("portfolio.typeAction"),
      etf: t("portfolio.typeEtf"),
      crypto: t("portfolio.typeCrypto"),
      autre: t("portfolio.typeOther"),
    };
    return m[k] ?? k;
  }

  const invested = $derived(holdings.reduce((s, h) => s + h.quantite * h.pru, 0));
  const currentValue = $derived(holdings.reduce((s, h) => s + h.quantite * h.prixActuel, 0));
  const gain = $derived(currentValue - invested);
  const gainPct = $derived(invested > 0 ? gain / invested : 0);

  const allocation = $derived.by(() => {
    const colors: Record<string, string> = {
      action: "var(--accent)",
      etf: "var(--gain)",
      crypto: "var(--soon)",
      autre: "var(--muted)",
    };
    const sums: Record<string, number> = {};
    for (const h of holdings) sums[h.type] = (sums[h.type] || 0) + h.quantite * h.prixActuel;
    return Object.entries(sums)
      .filter(([, v]) => v > 0)
      .map(([k, v]) => ({ label: typeLabel(k), value: v, color: colors[k] ?? "var(--muted)" }));
  });

  const val = (h: Holding) => h.quantite * h.prixActuel;
  const gl = (h: Holding) => h.quantite * (h.prixActuel - h.pru);
  const glPct = (h: Holding) => {
    const inv = h.quantite * h.pru;
    return inv > 0 ? gl(h) / inv : 0;
  };
  const tone = (x: number): "gain" | "loss" | "neutral" =>
    x > 0 ? "gain" : x < 0 ? "loss" : "neutral";

  function startAdd() {
    editingId = null;
    form = emptyForm();
    showForm = true;
  }
  function startEdit(h: Holding) {
    editingId = h.id;
    form = {
      nom: h.nom,
      symbole: h.symbole,
      type: h.type,
      quantite: h.quantite,
      pru: h.pru,
      prixActuel: h.prixActuel,
      devise: h.devise,
    };
    showForm = true;
  }

  const valid = $derived(form.nom.trim() !== "" || form.symbole.trim() !== "");

  async function submit() {
    if (saving || !valid) return;
    saving = true;
    try {
      const clean: NewHolding = {
        ...form,
        nom: form.nom.trim() || form.symbole.trim(),
        symbole: form.symbole.trim(),
        quantite: Number(form.quantite) || 0,
        pru: Number(form.pru) || 0,
        prixActuel: Number(form.prixActuel) || 0,
      };
      if (editingId != null) await updateHolding({ id: editingId, ...clean });
      else await addHolding(clean);
      showForm = false;
      await refresh();
    } finally {
      saving = false;
    }
  }

  async function remove(h: Holding) {
    await deleteHolding(h.id);
    if (editingId === h.id) showForm = false;
    await refresh();
  }

  function exportJson() {
    const blob = new Blob([JSON.stringify(holdings, null, 2)], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = "pecule-portefeuille.json";
    a.click();
    URL.revokeObjectURL(url);
  }
</script>

<div class="page-head">
  <h1 class="h1">{t("portfolio.title")}</h1>
  <p class="small muted">{t("portfolio.manualNote")}</p>
</div>

{#if loaded}
  <div class="stack-6">
    {#if showForm}
      <div class="card pad form-card">
        <h2 class="h2">{editingId != null ? t("portfolio.edit") : t("portfolio.addPosition")}</h2>
        <div class="form-grid">
          <label class="fld">
            <span class="small muted">{t("portfolio.name")}</span>
            <input class="in" bind:value={form.nom} placeholder="Air Liquide" />
          </label>
          <label class="fld">
            <span class="small muted">{t("portfolio.symbol")}</span>
            <input class="in mono" bind:value={form.symbole} placeholder="AI.PA" />
          </label>
          <div class="fld">
            <span class="small muted">{t("portfolio.assetType")}</span>
            <SegmentedControl
              options={typeOptions}
              bind:value={form.type}
              size="sm"
              ariaLabel={t("portfolio.assetType")}
            />
          </div>
          <label class="fld">
            <span class="small muted">{t("portfolio.quantity")}</span>
            <input class="in mono" type="number" min="0" step="any" bind:value={form.quantite} />
          </label>
          <label class="fld">
            <span class="small muted">{t("portfolio.pru")} (€)</span>
            <input class="in mono" type="number" min="0" step="any" bind:value={form.pru} />
          </label>
          <label class="fld">
            <span class="small muted">{t("portfolio.currentPrice")} (€)</span>
            <input class="in mono" type="number" min="0" step="any" bind:value={form.prixActuel} />
          </label>
        </div>
        <div class="form-actions">
          <button class="btn ghost" type="button" onclick={() => (showForm = false)}
            >{t("portfolio.cancel")}</button
          >
          <button class="btn cta" type="button" onclick={submit} disabled={!valid || saving}>
            {editingId != null ? t("portfolio.save") : t("portfolio.add")}
          </button>
        </div>
      </div>
    {/if}

    {#if holdings.length === 0 && !showForm}
      <EmptyState
        icon={Wallet}
        title={t("portfolio.emptyTitle")}
        body={t("portfolio.emptyBody")}
      />
      <div>
        <button class="btn cta" type="button" onclick={startAdd}>
          <Plus size={16} aria-hidden="true" />
          {t("portfolio.addPosition")}
        </button>
      </div>
    {:else if holdings.length > 0}
      <Card>
        <div class="statgrid">
          <Stat label={t("portfolio.invested")} value={fmtEUR(invested)} />
          <Stat label={t("portfolio.currentValue")} value={fmtEUR(currentValue)} tone="accent" />
          <Stat
            label={t("portfolio.gainLoss")}
            value={fmtSignedEUR(gain)}
            sub={fmtSignedPct(gainPct)}
            tone={tone(gain)}
          />
        </div>
        {#if allocation.length}
          <div class="alloc">
            <span class="small muted">{t("portfolio.allocation")}</span>
            <CostBar segments={allocation} format={fmtEUR} ariaLabel={t("portfolio.allocation")} />
          </div>
        {/if}
      </Card>

      <Card title={t("portfolio.positions")}>
        {#snippet actions()}
          <div class="hdr-actions">
            <button class="btn ghost sm" type="button" onclick={exportJson}>
              <Download size={14} aria-hidden="true" />
              {t("portfolio.exportJson")}
            </button>
            <button class="btn cta sm" type="button" onclick={startAdd}>
              <Plus size={14} aria-hidden="true" />
              {t("portfolio.addPosition")}
            </button>
          </div>
        {/snippet}

        <div class="table-scroll">
          <table class="pf">
            <thead>
              <tr>
                <th>{t("portfolio.name")}</th>
                <th>{t("portfolio.assetType")}</th>
                <th class="num">{t("portfolio.quantity")}</th>
                <th class="num">{t("portfolio.pru")}</th>
                <th class="num">{t("portfolio.currentPrice")}</th>
                <th class="num">{t("portfolio.value")}</th>
                <th class="num">{t("portfolio.gainLoss")}</th>
                <th aria-label="actions"></th>
              </tr>
            </thead>
            <tbody>
              {#each holdings as h (h.id)}
                <tr>
                  <td>
                    <div class="nm">{h.nom || h.symbole}</div>
                    {#if h.symbole && h.nom}<div class="sym small muted mono">{h.symbole}</div>{/if}
                  </td>
                  <td><span class="tag">{typeLabel(h.type)}</span></td>
                  <td class="num mono">{fmtNum(h.quantite, 2)}</td>
                  <td class="num mono">{fmtEUR(h.pru)}</td>
                  <td class="num mono">{fmtEUR(h.prixActuel)}</td>
                  <td class="num mono strong">{fmtEUR(val(h))}</td>
                  <td class="num mono {tone(gl(h))}">
                    {fmtSignedEUR(gl(h))}
                    <span class="pct small">({fmtSignedPct(glPct(h))})</span>
                  </td>
                  <td class="row-actions">
                    <button class="icon" type="button" onclick={() => startEdit(h)} aria-label={t("portfolio.edit")}>
                      <Pencil size={15} aria-hidden="true" />
                    </button>
                    <button class="icon" type="button" onclick={() => remove(h)} aria-label={t("portfolio.delete")}>
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
  .btn.ghost:hover {
    background: var(--surface);
  }
  .hdr-actions {
    display: flex;
    gap: var(--space-2);
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
  table.pf {
    width: 100%;
    border-collapse: collapse;
    font-size: var(--fs-small);
  }
  .pf th {
    text-align: left;
    font-weight: 500;
    color: var(--muted);
    padding: 8px 10px;
    border-bottom: 1px solid var(--line);
    white-space: nowrap;
  }
  .pf td {
    padding: 10px;
    border-bottom: 1px solid var(--line);
    vertical-align: middle;
  }
  .pf tbody tr:last-child td {
    border-bottom: 0;
  }
  .pf .num {
    text-align: right;
    white-space: nowrap;
  }
  .pf .strong {
    color: var(--ink);
    font-weight: 500;
  }
  .nm {
    color: var(--ink);
    font-weight: 500;
  }
  .sym {
    margin-top: 1px;
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
  .pct {
    opacity: 0.85;
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
