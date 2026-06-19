// Point d'entrée du module i18n : ré-exporte l'état réactif (runes) afin que les
// imports `$lib/i18n` se résolvent à un fichier `.ts` standard.
export { loc, setLocale, t, type Locale } from "./state.svelte";
