// Formatage localisé des montants, pourcentages et nombres (§22).
// Les fonctions lisent la langue courante (réactive) pour choisir le format Intl.
import { loc } from "./i18n";

const INTL_LOCALE: Record<string, string> = { fr: "fr-FR", en: "en-GB" };

function L(): string {
  return INTL_LOCALE[loc()] ?? "fr-FR";
}

/** Montant en euros, 2 décimales par défaut. */
export function fmtEUR(n: number, digits = 2): string {
  return new Intl.NumberFormat(L(), {
    style: "currency",
    currency: "EUR",
    minimumFractionDigits: digits,
    maximumFractionDigits: digits,
  }).format(safe(n));
}

/** Montant en euros sans décimales (gros chiffres de tableau de bord). */
export function fmtEUR0(n: number): string {
  return fmtEUR(n, 0);
}

/** Montant signé (+/−) en euros — pour les plus/moins-values. */
export function fmtSignedEUR(n: number, digits = 2): string {
  return new Intl.NumberFormat(L(), {
    style: "currency",
    currency: "EUR",
    signDisplay: "always",
    minimumFractionDigits: digits,
    maximumFractionDigits: digits,
  }).format(safe(n));
}

/** Pourcentage à partir d'un décimal (0.05 → « 5 % »). */
export function fmtPct(decimal: number, digits = 2): string {
  return new Intl.NumberFormat(L(), {
    style: "percent",
    minimumFractionDigits: 0,
    maximumFractionDigits: digits,
  }).format(safe(decimal));
}

/** Pourcentage signé à partir d'un décimal. */
export function fmtSignedPct(decimal: number, digits = 1): string {
  return new Intl.NumberFormat(L(), {
    style: "percent",
    signDisplay: "always",
    minimumFractionDigits: 0,
    maximumFractionDigits: digits,
  }).format(safe(decimal));
}

/** Nombre simple (séparateurs de milliers). */
export function fmtNum(n: number, digits = 0): string {
  return new Intl.NumberFormat(L(), {
    minimumFractionDigits: digits,
    maximumFractionDigits: digits,
  }).format(safe(n));
}

function safe(n: number): number {
  return Number.isFinite(n) ? n : 0;
}
