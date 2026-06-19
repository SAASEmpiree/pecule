// Pont IPC typé vers le backend Rust (commandes Tauri). Les noms de champs sont
// en camelCase pour coller à serde(rename_all = "camelCase").
//
// En application native : appelle le moteur Rust (source de vérité). Hors Tauri
// (prévisualisation navigateur) : retombe sur le moteur TS de `sim-fallback`.
//
// Toutes les entrées numériques sont **assainies** ici : un champ `<input number>`
// vidé donne `null`/`NaN`, ce qui ferait échouer la désérialisation `f64` côté
// Rust (invoke rejeté). On garantit donc des nombres finis avant tout appel.
import { invoke } from "@tauri-apps/api/core";
import * as fallback from "./sim-fallback";

export interface SeriesPoint {
  year: number;
  value: number;
  contributed: number;
  interest: number;
}

export interface CompoundParams {
  principal: number;
  annualRate: number;
  years: number;
  compoundsPerYear: number;
}
export interface CompoundResult {
  finalValue: number;
  totalContributed: number;
  totalInterest: number;
  series: SeriesPoint[];
}

export type Timing = "start" | "end";
export interface SavingsParams {
  initial: number;
  monthlyContribution: number;
  annualRate: number;
  years: number;
  timing: Timing;
}
export interface SavingsResult {
  finalValue: number;
  totalContributed: number;
  totalInterest: number;
  series: SeriesPoint[];
}

export interface LoanParams {
  principal: number;
  annualRate: number;
  termMonths: number;
  insuranceAnnualRate: number;
}
export interface AmortRow {
  month: number;
  payment: number;
  principalPart: number;
  interestPart: number;
  insurance: number;
  balance: number;
}
export interface LoanResult {
  monthlyPayment: number;
  monthlyInsurance: number;
  monthlyTotal: number;
  totalInterest: number;
  totalInsurance: number;
  totalCost: number;
  schedule: AmortRow[];
}

/** Vrai si l'on tourne dans l'application native Tauri. */
export function isTauri(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

/** Coerce une entrée en nombre fini (un `<input number>` vidé donne null/NaN). */
function num(x: unknown, fallbackValue = 0): number {
  const n = typeof x === "number" ? x : Number(x);
  return Number.isFinite(n) ? n : fallbackValue;
}

export async function simulateCompound(p: CompoundParams): Promise<CompoundResult> {
  const params: CompoundParams = {
    principal: num(p.principal),
    annualRate: num(p.annualRate),
    years: num(p.years),
    compoundsPerYear: Math.max(1, Math.trunc(num(p.compoundsPerYear, 1))),
  };
  return isTauri()
    ? invoke<CompoundResult>("simulate_compound", { params })
    : fallback.compound(params);
}

export async function simulateSavings(p: SavingsParams): Promise<SavingsResult> {
  const params: SavingsParams = {
    initial: num(p.initial),
    monthlyContribution: num(p.monthlyContribution),
    annualRate: num(p.annualRate),
    years: num(p.years),
    timing: p.timing === "start" ? "start" : "end",
  };
  return isTauri()
    ? invoke<SavingsResult>("simulate_savings", { params })
    : fallback.savings(params);
}

export async function simulateLoan(p: LoanParams): Promise<LoanResult> {
  const params: LoanParams = {
    principal: num(p.principal),
    annualRate: num(p.annualRate),
    termMonths: Math.max(0, Math.trunc(num(p.termMonths))),
    insuranceAnnualRate: num(p.insuranceAnnualRate),
  };
  return isTauri()
    ? invoke<LoanResult>("simulate_loan", { params })
    : fallback.loan(params);
}

// — Réglages persistés (table settings) — disponibles uniquement en natif. —
export async function getSetting(key: string): Promise<string | null> {
  if (!isTauri()) {
    try {
      return localStorage.getItem(`pecule:setting:${key}`);
    } catch {
      return null;
    }
  }
  return invoke<string | null>("get_setting", { key });
}

export async function setSetting(key: string, value: string): Promise<void> {
  if (!isTauri()) {
    try {
      localStorage.setItem(`pecule:setting:${key}`, value);
    } catch {
      /* ignore */
    }
    return;
  }
  await invoke("set_setting", { key, value });
}
