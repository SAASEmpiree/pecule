// Pont IPC typé vers le backend Rust (commandes Tauri). Les noms de champs sont
// en camelCase pour coller à serde(rename_all = "camelCase").
//
// En application native : appelle le moteur Rust (source de vérité). Hors Tauri
// (prévisualisation navigateur) : retombe sur le moteur TS de `sim-fallback`.
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

export async function simulateCompound(params: CompoundParams): Promise<CompoundResult> {
  return isTauri()
    ? invoke<CompoundResult>("simulate_compound", { params })
    : fallback.compound(params);
}

export async function simulateSavings(params: SavingsParams): Promise<SavingsResult> {
  return isTauri()
    ? invoke<SavingsResult>("simulate_savings", { params })
    : fallback.savings(params);
}

export async function simulateLoan(params: LoanParams): Promise<LoanResult> {
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
