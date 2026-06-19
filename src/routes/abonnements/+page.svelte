<script lang="ts">
  import { Plus, Pencil, Trash2, ArrowRight } from "@lucide/svelte";
  import Card from "$lib/components/Card.svelte";
  import Stat from "$lib/components/Stat.svelte";
  import Disclaimer from "$lib/components/Disclaimer.svelte";
  import EmptyState from "$lib/components/EmptyState.svelte";
  import SegmentedControl from "$lib/components/SegmentedControl.svelte";
  import { CalendarClock } from "@lucide/svelte";
  import {
    listSubscriptions,
    addSubscription,
    updateSubscription,
    deleteSubscription,
    type Subscription,
    type NewSubscription,
  } from "$lib/ipc";
  import { t } from "$lib/i18n";
  import { fmtEUR } from "$lib/format";

  let subs = $state<Subscription[]>([]);
  let loaded = $state(false);
  let showForm = $state(false);
  let editingId = $state<number | null>(null);
  let saving = $state(false);

  type FormData = {
    nom: string;
    categorie: string;
    montant: number;
    frequence: string;
    prochainPrelevement: string;
    actif: boolean;
  };
  const emptyForm = (): FormData => ({
    nom: "",
    categorie: "",
    montant: 0,
    frequence: "mensuel",
    prochainPrelevement: "",
    actif: true,
  });
  let form = $state<FormData>(emptyForm());

  async function refresh() {
    subs = await listSubscriptions();
    loaded = true;
  }
  $effect(() => {
    refresh();
  });

  const freqOptions = $derived([
    { value: "mensuel", label: t("subscriptions.freqMensuel") },
    { value: "annuel", label: t("subscriptions.freqAnnuel") },
    { value: "trimestriel", label: t("subscriptions.freqTrimestriel") },
    { value: "hebdomadaire", label: t("subscriptions.freqHebdomadaire") },
  ]);
  function freqLabel(f: string): string {
    const m: Record<string, string> = {
      mensuel: t("subscriptions.freqMensuel"),
      annuel: t("subscriptions.freqAnnuel"),
      trimestriel: t("subscriptions.freqTrimestriel"),
      hebdomadaire: t("subscriptions.freqHebdomadaire"),
    };
    return m[f] ?? f;
  }

  const FACTORS: Record<string, number> = {
    mensuel: 1,
    annuel: 1 / 12,
    trimestriel: 1 / 3,
    hebdomadaire: 52 / 12,
  };
  const monthly = (s: Subscription) => (s.actif ? s.montant * (FACTORS[s.frequence] ?? 1) : 0);

  const monthlyTotal = $derived(subs.reduce((sum, s) => sum + monthly(s), 0));
  const annualTotal = $derived(monthlyTotal * 12);

  function daysUntil(iso: string | null): number | null {
    if (!iso) return null;
    const d = new Date(iso + "T00:00:00");
    if (Number.isNaN(d.getTime())) return null;
    const today = new Date();
    today.setHours(0, 0, 0, 0);
    return Math.round((d.getTime() - today.getTime()) / 86400000);
  }

  const upcoming = $derived(
    subs
      .filter((s) => s.actif && s.prochainPrelevement)
      .map((s) => ({ s, d: daysUntil(s.prochainPrelevement) }))
      .filter((x) => x.d !== null && (x.d as number) >= -3 && (x.d as number) <= 14)
      .sort((a, b) => (a.d as number) - (b.d as number)),
  );

  function renewalLabel(d: number | null): string {
    if (d === null) return "";
    if (d < 0) return t("subscriptions.overdue");
    if (d === 0) return t("subscriptions.today");
    return t("subscriptions.inDays", { n: d });
  }
  function renewalTone(d: number | null): "soon" | "loss" | "muted" {
    if (d === null) return "muted";
    if (d < 0) return "loss";
    if (d <= 7) return "soon";
    return "muted";
  }
  // Ton compatible avec le composant Stat (pas de « muted »).
  const upcomingTone = $derived<"neutral" | "soon" | "loss">(
    upcoming.length === 0
      ? "neutral"
      : renewalTone(upcoming[0].d) === "muted"
        ? "neutral"
        : (renewalTone(upcoming[0].d) as "soon" | "loss"),
  );

  function startAdd() {
    editingId = null;
    form = emptyForm();
    showForm = true;
  }
  function startEdit(s: Subscription) {
    editingId = s.id;
    form = {
      nom: s.nom,
      categorie: s.categorie ?? "",
      montant: s.montant,
      frequence: s.frequence,
      prochainPrelevement: s.prochainPrelevement ?? "",
      actif: s.actif,
    };
    showForm = true;
  }

  const valid = $derived(form.nom.trim() !== "");

  async function submit() {
    if (saving || !valid) return;
    saving = true;
    try {
      const clean: NewSubscription = {
        nom: form.nom.trim(),
        categorie: form.categorie.trim() || null,
        montant: Number(form.montant) || 0,
        frequence: form.frequence,
        prochainPrelevement: form.prochainPrelevement || null,
        actif: form.actif,
      };
      if (editingId != null) await updateSubscription({ id: editingId, ...clean });
      else await addSubscription(clean);
      showForm = false;
      await refresh();
    } finally {
      saving = false;
    }
  }

  async function remove(s: Subscription) {
    await deleteSubscription(s.id);
    if (editingId === s.id) showForm = false;
    await refresh();
  }
</script>

<div class="page-head">
  <h1 class="h1">{t("subscriptions.title")}</h1>
</div>

{#if loaded}
  <div class="stack-6">
    {#if showForm}
      <div class="card pad form-card">
        <h2 class="h2">
          {editingId != null ? t("subscriptions.edit") : t("subscriptions.addSubscription")}
        </h2>
        <div class="form-grid">
          <label class="fld">
            <span class="small muted">{t("subscriptions.name")}</span>
            <input class="in" bind:value={form.nom} placeholder="Spotify" />
          </label>
          <label class="fld">
            <span class="small muted">{t("subscriptions.category")}</span>
            <input class="in" bind:value={form.categorie} placeholder="Musique" />
          </label>
          <label class="fld">
            <span class="small muted">{t("subscriptions.amount")} (€)</span>
            <input class="in mono" type="number" min="0" step="any" bind:value={form.montant} />
          </label>
          <div class="fld">
            <span class="small muted">{t("subscriptions.frequency")}</span>
            <SegmentedControl
              options={freqOptions}
              bind:value={form.frequence}
              size="sm"
              ariaLabel={t("subscriptions.frequency")}
            />
          </div>
          <label class="fld">
            <span class="small muted">{t("subscriptions.nextRenewal")}</span>
            <input class="in mono" type="date" bind:value={form.prochainPrelevement} />
          </label>
          <label class="fld check">
            <input type="checkbox" bind:checked={form.actif} />
            <span class="small">{t("subscriptions.active")}</span>
          </label>
        </div>
        <div class="form-actions">
          <button class="btn ghost" type="button" onclick={() => (showForm = false)}>
            {t("subscriptions.cancel")}
          </button>
          <button class="btn cta" type="button" onclick={submit} disabled={!valid || saving}>
            {editingId != null ? t("subscriptions.save") : t("subscriptions.addSubscription")}
          </button>
        </div>
      </div>
    {/if}

    {#if subs.length === 0 && !showForm}
      <EmptyState
        icon={CalendarClock}
        title={t("subscriptions.emptyTitle")}
        body={t("subscriptions.emptyBody")}
      />
      <div>
        <button class="btn cta" type="button" onclick={startAdd}>
          <Plus size={16} aria-hidden="true" />
          {t("subscriptions.addSubscription")}
        </button>
      </div>
    {:else if subs.length > 0}
      <Card>
        <div class="statgrid">
          <Stat label={t("subscriptions.monthlyTotal")} value={fmtEUR(monthlyTotal)} tone="accent" />
          <Stat label={t("subscriptions.annualTotal")} value={fmtEUR(annualTotal)} />
          <Stat
            label={t("subscriptions.upcoming")}
            value={upcoming.length > 0
              ? `${upcoming[0].s.nom} · ${renewalLabel(upcoming[0].d)}`
              : t("subscriptions.noUpcoming")}
            tone={upcomingTone}
          />
        </div>
        <a class="whatif" href="/simulateurs/epargne">
          {t("subscriptions.whatIf")}
          <ArrowRight size={14} aria-hidden="true" />
        </a>
      </Card>

      <Card title={t("subscriptions.title")}>
        {#snippet actions()}
          <button class="btn cta sm" type="button" onclick={startAdd}>
            <Plus size={14} aria-hidden="true" />
            {t("subscriptions.addSubscription")}
          </button>
        {/snippet}
        <div class="table-scroll">
          <table class="subs">
            <thead>
              <tr>
                <th>{t("subscriptions.name")}</th>
                <th>{t("subscriptions.frequency")}</th>
                <th class="num">{t("subscriptions.amount")}</th>
                <th class="num">{t("subscriptions.monthlyTotal")}</th>
                <th>{t("subscriptions.nextRenewal")}</th>
                <th aria-label="actions"></th>
              </tr>
            </thead>
            <tbody>
              {#each subs as s (s.id)}
                {@const d = daysUntil(s.prochainPrelevement)}
                <tr class:dim={!s.actif}>
                  <td>
                    <div class="nm">{s.nom}</div>
                    {#if s.categorie}<div class="small muted">{s.categorie}</div>{/if}
                  </td>
                  <td><span class="tag">{freqLabel(s.frequence)}</span></td>
                  <td class="num mono">{fmtEUR(s.montant)}</td>
                  <td class="num mono strong">{s.actif ? fmtEUR(monthly(s)) : "—"}</td>
                  <td>
                    {#if s.prochainPrelevement}
                      <span class="chip {renewalTone(d)}">{renewalLabel(d)}</span>
                    {:else}
                      <span class="small muted">—</span>
                    {/if}
                  </td>
                  <td class="row-actions">
                    <button class="icon" type="button" onclick={() => startEdit(s)} aria-label={t("subscriptions.edit")}>
                      <Pencil size={15} aria-hidden="true" />
                    </button>
                    <button class="icon" type="button" onclick={() => remove(s)} aria-label={t("subscriptions.delete")}>
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
  .whatif {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    margin-top: var(--space-5);
    color: var(--accent);
    font-weight: 500;
    font-size: var(--fs-small);
  }
  .whatif:hover {
    text-decoration: underline;
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
  .fld.check {
    flex-direction: row;
    align-items: center;
    gap: var(--space-2);
    align-self: end;
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
  table.subs {
    width: 100%;
    border-collapse: collapse;
    font-size: var(--fs-small);
  }
  .subs th {
    text-align: left;
    font-weight: 500;
    color: var(--muted);
    padding: 8px 10px;
    border-bottom: 1px solid var(--line);
    white-space: nowrap;
  }
  .subs td {
    padding: 10px;
    border-bottom: 1px solid var(--line);
    vertical-align: middle;
  }
  .subs tbody tr:last-child td {
    border-bottom: 0;
  }
  .subs tr.dim {
    opacity: 0.5;
  }
  .subs .num {
    text-align: right;
    white-space: nowrap;
  }
  .subs .strong {
    color: var(--ink);
    font-weight: 500;
  }
  .nm {
    color: var(--ink);
    font-weight: 500;
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
  .chip {
    display: inline-block;
    padding: 2px 9px;
    border-radius: var(--radius-pill);
    font-size: 11px;
    font-weight: 600;
    white-space: nowrap;
  }
  .chip.soon {
    background: var(--soon-soft);
    color: var(--soon);
  }
  .chip.loss {
    background: var(--loss-soft);
    color: var(--loss);
  }
  .chip.muted {
    background: var(--surface-2);
    color: var(--muted);
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
