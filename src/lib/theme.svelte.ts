// État de thème réactif (Svelte 5 runes). Sombre par défaut (§4.1), persisté en
// localStorage, appliqué via l'attribut data-theme sur <html>.
import { browser } from "$app/environment";

export type Theme = "dark" | "light";
const KEY = "pecule:theme";

let current = $state<Theme>("dark");

if (browser) {
  try {
    const saved = localStorage.getItem(KEY);
    if (saved === "light" || saved === "dark") current = saved;
  } catch {
    /* localStorage indisponible : on garde le défaut */
  }
  applyTheme(current);
}

function applyTheme(t: Theme) {
  if (!browser) return;
  if (t === "light") document.documentElement.setAttribute("data-theme", "light");
  else document.documentElement.removeAttribute("data-theme");
}

/** Lecture réactive du thème courant. */
export function theme(): Theme {
  return current;
}

export function setTheme(t: Theme) {
  current = t;
  applyTheme(t);
  if (browser) {
    try {
      localStorage.setItem(KEY, t);
    } catch {
      /* ignore */
    }
  }
}

export function toggleTheme() {
  setTheme(current === "dark" ? "light" : "dark");
}
