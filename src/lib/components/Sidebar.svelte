<script lang="ts">
  import { page } from "$app/stores";
  import {
    LayoutDashboard,
    Wallet,
    Calculator,
    CalendarClock,
    Settings,
    TrendingUp,
    PiggyBank,
    Landmark,
    Flame,
    Coins,
  } from "@lucide/svelte";
  import { t } from "$lib/i18n";

  const primary = [
    { href: "/", icon: LayoutDashboard, key: "nav.dashboard" },
    { href: "/patrimoine", icon: Coins, key: "nav.networth" },
    { href: "/portefeuille", icon: Wallet, key: "nav.portfolio" },
  ];
  const sims = [
    { href: "/simulateurs/interets-composes", icon: TrendingUp, key: "nav.simCompound" },
    { href: "/simulateurs/epargne", icon: PiggyBank, key: "nav.simSavings" },
    { href: "/simulateurs/pret", icon: Landmark, key: "nav.simLoan" },
    { href: "/simulateurs/fire", icon: Flame, key: "nav.simFire", soon: true },
  ];
  const secondary = [
    { href: "/abonnements", icon: CalendarClock, key: "nav.subscriptions" },
    { href: "/reglages", icon: Settings, key: "nav.settings" },
  ];

  function isActive(href: string, pathname: string): boolean {
    return href === "/" ? pathname === "/" : pathname.startsWith(href);
  }
</script>

<nav class="sidebar" aria-label={t("nav.dashboard")}>
  <ul class="group">
    {#each primary as item (item.href)}
      {@const Icon = item.icon}
      <li>
        <a
          href={item.href}
          class="item"
          class:active={isActive(item.href, $page.url.pathname)}
          aria-current={isActive(item.href, $page.url.pathname) ? "page" : undefined}
        >
          <Icon size={18} aria-hidden="true" />
          <span>{t(item.key)}</span>
        </a>
      </li>
    {/each}
  </ul>

  <div class="section">
    <span class="section-title small muted">
      <Calculator size={13} aria-hidden="true" />
      {t("nav.simulators")}
    </span>
    <ul class="group sub">
      {#each sims as item (item.href)}
        {@const Icon = item.icon}
        <li>
          {#if item.soon}
            <span class="item disabled">
              <Icon size={17} aria-hidden="true" />
              <span>{t(item.key)}</span>
              <span class="badge small">{t("common.soon")}</span>
            </span>
          {:else}
            <a
              href={item.href}
              class="item"
              class:active={isActive(item.href, $page.url.pathname)}
              aria-current={isActive(item.href, $page.url.pathname) ? "page" : undefined}
            >
              <Icon size={17} aria-hidden="true" />
              <span>{t(item.key)}</span>
            </a>
          {/if}
        </li>
      {/each}
    </ul>
  </div>

  <ul class="group bottom">
    {#each secondary as item (item.href)}
      {@const Icon = item.icon}
      <li>
        <a
          href={item.href}
          class="item"
          class:active={isActive(item.href, $page.url.pathname)}
          aria-current={isActive(item.href, $page.url.pathname) ? "page" : undefined}
        >
          <Icon size={18} aria-hidden="true" />
          <span>{t(item.key)}</span>
        </a>
      </li>
    {/each}
  </ul>
</nav>

<style>
  .sidebar {
    grid-area: sidebar;
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    padding: var(--space-4) var(--space-3);
    border-right: 1px solid var(--line);
    background: var(--surface);
    overflow-y: auto;
  }
  .group {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }
  .section {
    margin-top: var(--space-3);
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }
  .section-title {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: var(--space-2) var(--space-3) var(--space-1);
    text-transform: uppercase;
    letter-spacing: 0.06em;
    font-size: 11px;
  }
  .sub .item {
    padding-left: var(--space-4);
  }
  .bottom {
    margin-top: auto;
  }
  .item {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: 9px var(--space-3);
    border-radius: var(--radius-sm);
    color: var(--muted);
    font-size: var(--fs-body);
    font-weight: 500;
    position: relative;
    transition: background var(--dur-fast) var(--ease),
      color var(--dur-fast) var(--ease);
  }
  .item:hover {
    background: var(--surface-2);
    color: var(--ink);
  }
  .item.active {
    background: var(--accent-soft);
    color: var(--accent);
  }
  .item.active::before {
    content: "";
    position: absolute;
    left: 0;
    top: 18%;
    bottom: 18%;
    width: 3px;
    border-radius: 999px;
    background: var(--accent);
  }
  .item.disabled {
    color: var(--muted);
    opacity: 0.55;
    cursor: default;
    pointer-events: none;
  }
  .badge {
    margin-left: auto;
    background: var(--soon-soft);
    color: var(--soon);
    padding: 1px 7px;
    border-radius: var(--radius-pill);
    font-size: 10px;
    font-weight: 600;
  }
</style>
