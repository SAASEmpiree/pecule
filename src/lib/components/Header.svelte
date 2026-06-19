<script lang="ts">
  import { Lock, Sun, Moon, Languages } from "@lucide/svelte";
  import Logo from "./Logo.svelte";
  import { theme, toggleTheme } from "$lib/theme.svelte";
  import { loc, setLocale, t } from "$lib/i18n";
</script>

<header class="header" data-tauri-drag-region>
  <a class="brand" href="/">
    <Logo size={24} />
    <span class="name">{t("app.name")}</span>
  </a>

  <a class="privacy" href="/reglages" title={t("privacy.title")}>
    <Lock size={13} aria-hidden="true" />
    <span class="small">{t("header.privacy")}</span>
  </a>

  <div class="controls">
    <button
      class="icon-btn"
      type="button"
      onclick={toggleTheme}
      aria-label={t("header.theme")}
      title={t("header.theme")}
    >
      {#if theme() === "dark"}
        <Sun size={18} aria-hidden="true" />
      {:else}
        <Moon size={18} aria-hidden="true" />
      {/if}
    </button>
    <button
      class="lang"
      type="button"
      onclick={() => setLocale(loc() === "fr" ? "en" : "fr")}
      aria-label={t("header.language")}
      title={t("header.language")}
    >
      <Languages size={16} aria-hidden="true" />
      <span class="small mono">{loc().toUpperCase()}</span>
    </button>
  </div>
</header>

<style>
  .header {
    grid-area: header;
    display: flex;
    align-items: center;
    gap: var(--space-4);
    padding: 0 var(--space-4);
    border-bottom: 1px solid var(--line);
    background: var(--surface);
  }
  /* Sur macOS uniquement (barre Overlay), réserver la place des feux natifs. */
  :global(html.is-macos) .header {
    padding-left: 78px;
  }
  .brand {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    color: var(--ink);
  }
  .name {
    font-size: 17px;
    font-weight: 600;
    letter-spacing: -0.01em;
  }
  .privacy {
    margin-inline: auto;
    display: flex;
    align-items: center;
    gap: var(--space-2);
    padding: 6px 12px;
    border-radius: var(--radius-pill);
    background: var(--surface-2);
    border: 1px solid var(--line);
    color: var(--muted);
    transition: color var(--dur-fast) var(--ease),
      border-color var(--dur-fast) var(--ease);
  }
  .privacy:hover {
    color: var(--accent);
    border-color: var(--accent);
  }
  .controls {
    display: flex;
    align-items: center;
    gap: var(--space-2);
  }
  .icon-btn,
  .lang {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    height: 34px;
    padding: 0 10px;
    border-radius: var(--radius-sm);
    border: 1px solid var(--line);
    background: var(--surface-2);
    color: var(--muted);
    transition: color var(--dur-fast) var(--ease),
      background var(--dur-fast) var(--ease);
  }
  .icon-btn {
    width: 34px;
    justify-content: center;
    padding: 0;
  }
  .icon-btn:hover,
  .lang:hover {
    color: var(--ink);
    background: var(--surface);
  }
  .lang .mono {
    font-weight: 600;
  }
</style>
