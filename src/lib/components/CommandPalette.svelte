<script lang="ts">
  import { goto } from "$app/navigation";
  import {
    Search,
    LayoutDashboard,
    Wallet,
    TrendingUp,
    PiggyBank,
    Landmark,
    CalendarClock,
    Settings,
    Plus,
    SunMoon,
    Languages,
    Coins,
  } from "@lucide/svelte";
  import type { Component } from "svelte";
  import { t, loc, setLocale } from "$lib/i18n";
  import { toggleTheme } from "$lib/theme.svelte";
  import { paletteIsOpen, openPalette, closePalette, togglePalette } from "$lib/palette.svelte";

  type Cmd = {
    id: string;
    label: string;
    group: string;
    icon: Component<any>;
    run: () => void;
    keepOpen?: boolean;
  };

  let query = $state("");
  let selected = $state(0);
  let inputEl: HTMLInputElement | null = $state(null);

  function go(href: string) {
    closePalette();
    goto(href);
  }

  const commands = $derived<Cmd[]>([
    { id: "dash", label: t("nav.dashboard"), group: t("palette.navigate"), icon: LayoutDashboard, run: () => go("/") },
    { id: "nw", label: t("nav.networth"), group: t("palette.navigate"), icon: Coins, run: () => go("/patrimoine") },
    { id: "pf", label: t("nav.portfolio"), group: t("palette.navigate"), icon: Wallet, run: () => go("/portefeuille") },
    { id: "subs", label: t("nav.subscriptions"), group: t("palette.navigate"), icon: CalendarClock, run: () => go("/abonnements") },
    { id: "sim-c", label: t("nav.simCompound"), group: t("palette.navigate"), icon: TrendingUp, run: () => go("/simulateurs/interets-composes") },
    { id: "sim-s", label: t("nav.simSavings"), group: t("palette.navigate"), icon: PiggyBank, run: () => go("/simulateurs/epargne") },
    { id: "sim-l", label: t("nav.simLoan"), group: t("palette.navigate"), icon: Landmark, run: () => go("/simulateurs/pret") },
    { id: "settings", label: t("nav.settings"), group: t("palette.navigate"), icon: Settings, run: () => go("/reglages") },
    { id: "add-pos", label: t("portfolio.addPosition"), group: t("palette.actions"), icon: Plus, run: () => go("/portefeuille") },
    { id: "theme", label: t("palette.toggleTheme"), group: t("palette.actions"), icon: SunMoon, run: () => toggleTheme(), keepOpen: true },
    { id: "lang", label: t("palette.toggleLang"), group: t("palette.actions"), icon: Languages, run: () => setLocale(loc() === "fr" ? "en" : "fr"), keepOpen: true },
  ]);

  function norm(s: string): string {
    return s
      .toLowerCase()
      .normalize("NFD")
      .replace(/\p{Diacritic}/gu, "");
  }

  const filtered = $derived.by(() => {
    const q = norm(query.trim());
    if (!q) return commands;
    const terms = q.split(/\s+/);
    return commands.filter((c) => {
      const hay = norm(`${c.label} ${c.group}`);
      return terms.every((tm) => hay.includes(tm));
    });
  });

  // Réinitialise la sélection quand la requête change.
  $effect(() => {
    query;
    selected = 0;
  });

  // Focus l'input à l'ouverture.
  $effect(() => {
    if (paletteIsOpen() && inputEl) inputEl.focus();
  });

  function run(cmd: Cmd) {
    cmd.run();
    if (!cmd.keepOpen) closePalette();
  }

  function onKeydown(e: KeyboardEvent) {
    if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === "k") {
      e.preventDefault();
      togglePalette();
      if (paletteIsOpen()) query = "";
      return;
    }
    if (!paletteIsOpen()) return;
    const list = filtered;
    if (e.key === "Escape") {
      e.preventDefault();
      closePalette();
    } else if (e.key === "ArrowDown") {
      e.preventDefault();
      selected = Math.min(selected + 1, list.length - 1);
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      selected = Math.max(selected - 1, 0);
    } else if (e.key === "Enter") {
      e.preventDefault();
      const c = list[Math.min(selected, list.length - 1)];
      if (c) run(c);
    }
  }
</script>

<svelte:window onkeydown={onKeydown} />

{#if paletteIsOpen()}
  <div class="overlay" role="presentation" onclick={closePalette}>
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <div
      class="palette"
      role="dialog"
      aria-modal="true"
      aria-label="Commandes"
      tabindex="-1"
      onclick={(e) => e.stopPropagation()}
    >
      <div class="search">
        <Search size={18} aria-hidden="true" />
        <input
          bind:this={inputEl}
          bind:value={query}
          placeholder={t("palette.placeholder")}
          aria-label={t("palette.placeholder")}
        />
        <kbd>esc</kbd>
      </div>
      <ul class="list">
        {#if filtered.length === 0}
          <li class="empty muted small">{t("palette.empty")}</li>
        {:else}
          {#each filtered as c, i (c.id)}
            {@const Icon = c.icon}
            <li>
              <button
                class="cmd"
                class:active={i === Math.min(selected, filtered.length - 1)}
                onmouseenter={() => (selected = i)}
                onclick={() => run(c)}
              >
                <Icon size={16} aria-hidden="true" />
                <span class="lbl">{c.label}</span>
                <span class="grp small muted">{c.group}</span>
              </button>
            </li>
          {/each}
        {/if}
      </ul>
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    z-index: 100;
    background: rgba(0, 0, 0, 0.45);
    backdrop-filter: blur(2px);
    display: flex;
    justify-content: center;
    align-items: flex-start;
    padding-top: 14vh;
  }
  .palette {
    width: min(560px, 92vw);
    max-height: 60vh;
    display: flex;
    flex-direction: column;
    background: var(--surface);
    border: 1px solid var(--line);
    border-radius: var(--radius-md);
    box-shadow: var(--shadow-pop);
    overflow: hidden;
  }
  .search {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: var(--space-4);
    border-bottom: 1px solid var(--line);
    color: var(--muted);
  }
  .search input {
    flex: 1;
    border: 0;
    background: transparent;
    color: var(--ink);
    font-size: var(--fs-h2);
    font-weight: 500;
    outline: none;
  }
  kbd {
    font-family: var(--font-mono);
    font-size: 11px;
    color: var(--muted);
    background: var(--surface-2);
    border: 1px solid var(--line);
    border-radius: 6px;
    padding: 2px 6px;
  }
  .list {
    list-style: none;
    margin: 0;
    padding: var(--space-2);
    overflow-y: auto;
  }
  .empty {
    padding: var(--space-4);
    text-align: center;
  }
  .cmd {
    width: 100%;
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: 10px var(--space-3);
    border: 0;
    border-radius: var(--radius-sm);
    background: transparent;
    color: var(--ink);
    text-align: left;
  }
  .cmd.active {
    background: var(--accent-soft);
    color: var(--accent);
  }
  .lbl {
    flex: 1;
    font-weight: 500;
  }
  .grp {
    flex-shrink: 0;
  }
</style>
