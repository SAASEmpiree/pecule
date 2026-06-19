<script lang="ts">
  import { TrendingUp, PiggyBank, Landmark, Flame } from "@lucide/svelte";
  import { t } from "$lib/i18n";

  const sims = $derived([
    {
      href: "/simulateurs/interets-composes",
      icon: TrendingUp,
      title: t("compound.title"),
      desc: t("compound.desc"),
      soon: false,
    },
    {
      href: "/simulateurs/epargne",
      icon: PiggyBank,
      title: t("savings.title"),
      desc: t("savings.desc"),
      soon: false,
    },
    {
      href: "/simulateurs/pret",
      icon: Landmark,
      title: t("loan.title"),
      desc: t("loan.desc"),
      soon: false,
    },
    {
      href: "/simulateurs/fire",
      icon: Flame,
      title: t("fire.title"),
      desc: t("fire.body"),
      soon: true,
    },
  ]);
</script>

<div class="page-head">
  <h1 class="h1">{t("simulatorsHub.title")}</h1>
  <p class="muted">{t("simulatorsHub.desc")}</p>
</div>

<div class="grid">
  {#each sims as s (s.href)}
    {@const Icon = s.icon}
    <a class="tile card pad" href={s.soon ? undefined : s.href} class:disabled={s.soon}>
      <div class="ic"><Icon size={22} aria-hidden="true" /></div>
      <h2 class="h2">
        {s.title}{#if s.soon}<span class="badge">{t("common.soon")}</span>{/if}
      </h2>
      <p class="small muted">{s.desc}</p>
    </a>
  {/each}
</div>

<style>
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(240px, 1fr));
    gap: var(--space-4);
  }
  .tile {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    color: var(--ink);
    transition: border-color var(--dur-fast) var(--ease),
      transform var(--dur-fast) var(--ease);
  }
  .tile:hover {
    border-color: var(--accent);
    transform: translateY(-2px);
  }
  .tile.disabled {
    pointer-events: none;
    opacity: 0.6;
  }
  .ic {
    width: 42px;
    height: 42px;
    display: grid;
    place-items: center;
    border-radius: var(--radius-md);
    background: var(--accent-soft);
    color: var(--accent);
    margin-bottom: var(--space-1);
  }
  .badge {
    margin-left: var(--space-2);
    background: var(--soon-soft);
    color: var(--soon);
    padding: 2px 8px;
    border-radius: var(--radius-pill);
    font-size: 11px;
    font-weight: 600;
    vertical-align: middle;
  }
  p {
    line-height: 1.5;
  }
</style>
