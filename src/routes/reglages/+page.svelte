<script lang="ts">
  import { Lock } from "@lucide/svelte";
  import Card from "$lib/components/Card.svelte";
  import SegmentedControl from "$lib/components/SegmentedControl.svelte";
  import Disclaimer from "$lib/components/Disclaimer.svelte";
  import { theme, setTheme, type Theme } from "$lib/theme.svelte";
  import { loc, setLocale, t, type Locale } from "$lib/i18n";

  const VERSION = "0.1.0";

  const themeOptions = $derived([
    { value: "dark", label: t("settings.dark") },
    { value: "light", label: t("settings.light") },
  ]);
  const langOptions = $derived([
    { value: "fr", label: t("settings.french") },
    { value: "en", label: t("settings.english") },
  ]);
</script>

<div class="page-head">
  <h1 class="h1">{t("settings.title")}</h1>
</div>

<div class="stack-6 narrow">
  <Card title={t("settings.appearance")}>
    <div class="rows">
      <div class="srow">
        <span>{t("settings.theme")}</span>
        <SegmentedControl
          options={themeOptions}
          value={theme()}
          onchange={(v) => setTheme(v as Theme)}
          size="sm"
          ariaLabel={t("settings.theme")}
        />
      </div>
      <div class="srow">
        <span>{t("settings.language")}</span>
        <SegmentedControl
          options={langOptions}
          value={loc()}
          onchange={(v) => setLocale(v as Locale)}
          size="sm"
          ariaLabel={t("settings.language")}
        />
      </div>
    </div>
  </Card>

  <Card title={t("settings.privacyTitle")}>
    <div class="privacy">
      <div class="ic"><Lock size={18} aria-hidden="true" /></div>
      <div class="stack">
        <p>{t("settings.privacyBody")}</p>
        <p class="small muted">{t("settings.quotesNuance")}</p>
      </div>
    </div>
  </Card>

  <Card title={t("settings.about")}>
    <div class="rows">
      <div class="srow">
        <span class="muted">{t("settings.version")}</span>
        <span class="mono">Pécule {VERSION}</span>
      </div>
      <Disclaimer long />
    </div>
  </Card>
</div>

<style>
  .narrow {
    max-width: 720px;
  }
  .rows {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }
  .srow {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-4);
  }
  .privacy {
    display: flex;
    gap: var(--space-4);
    align-items: flex-start;
  }
  .privacy .ic {
    flex-shrink: 0;
    width: 40px;
    height: 40px;
    display: grid;
    place-items: center;
    border-radius: var(--radius-md);
    background: var(--accent-soft);
    color: var(--accent);
  }
  .privacy p {
    line-height: 1.55;
    max-width: 60ch;
  }
</style>
