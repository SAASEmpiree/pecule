<script lang="ts">
  // Polices auto-hébergées (aucun CDN au runtime → promesse « 0 réseau »).
  import "@fontsource/hanken-grotesk/400.css";
  import "@fontsource/hanken-grotesk/500.css";
  import "@fontsource/hanken-grotesk/600.css";
  import "@fontsource/hanken-grotesk/700.css";
  import "@fontsource/ibm-plex-mono/400.css";
  import "@fontsource/ibm-plex-mono/500.css";

  import "$lib/styles/tokens.css";
  import "$lib/styles/global.css";

  import Header from "$lib/components/Header.svelte";
  import Sidebar from "$lib/components/Sidebar.svelte";
  import CommandPalette from "$lib/components/CommandPalette.svelte";
  // Importé pour appliquer le thème enregistré dès le chargement.
  import "$lib/theme.svelte";

  let { children } = $props();
</script>

<div class="shell">
  <Header />
  <Sidebar />
  <main class="main">
    <div class="container">
      {@render children()}
    </div>
  </main>
</div>

<CommandPalette />

<style>
  .shell {
    height: 100vh;
    display: grid;
    grid-template-rows: var(--header-h) 1fr;
    grid-template-columns: var(--sidebar-w) 1fr;
    grid-template-areas:
      "header header"
      "sidebar main";
  }
  .main {
    grid-area: main;
    overflow-y: auto;
    background: var(--bg);
  }
  .container {
    max-width: 1000px;
    margin: 0 auto;
    padding: var(--space-8) var(--space-8) var(--space-12);
  }
</style>
