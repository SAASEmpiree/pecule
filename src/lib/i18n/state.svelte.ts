// i18n minimal et réactif (Svelte 5 runes). FR prioritaire, EN partiel avec
// repli sur le FR pour toute clé manquante (§22).
//
// Ce fichier porte l'état réactif (runes -> extension .svelte.ts). Il est
// ré-exporté par ./index.ts pour que les imports `$lib/i18n` fonctionnent.
import { browser } from "$app/environment";
import { en, fr } from "./messages";

export type Locale = "fr" | "en";
const KEY = "pecule:locale";
const dicts: Record<Locale, unknown> = { fr, en };

let current = $state<Locale>("fr");

if (browser) {
  try {
    const saved = localStorage.getItem(KEY);
    if (saved === "fr" || saved === "en") current = saved;
  } catch {
    /* ignore */
  }
  document.documentElement.lang = current;
}

/** Lecture réactive de la langue courante. */
export function loc(): Locale {
  return current;
}

export function setLocale(l: Locale) {
  current = l;
  if (browser) {
    document.documentElement.lang = l;
    try {
      localStorage.setItem(KEY, l);
    } catch {
      /* ignore */
    }
  }
}

function lookup(dict: unknown, key: string): string | undefined {
  let node: unknown = dict;
  for (const part of key.split(".")) {
    if (node && typeof node === "object" && part in (node as Record<string, unknown>)) {
      node = (node as Record<string, unknown>)[part];
    } else {
      return undefined;
    }
  }
  return typeof node === "string" ? node : undefined;
}

/**
 * Traduit une clé pointée (ex: `nav.dashboard`), avec interpolation `{x}`.
 * Réactif : lit la langue courante, donc se met à jour au changement de langue.
 */
export function t(key: string, vars?: Record<string, string | number>): string {
  const l = current; // dépendance réactive
  let s = lookup(dicts[l], key) ?? lookup(dicts.fr, key);
  if (s == null) return key;
  if (vars) {
    for (const k of Object.keys(vars)) {
      s = s.replaceAll(`{${k}}`, String(vars[k]));
    }
  }
  return s;
}
