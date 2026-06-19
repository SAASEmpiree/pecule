// Repli de calcul en TypeScript, utilisé UNIQUEMENT pour la prévisualisation
// dans un navigateur (hors application native). Dans l'app Tauri, c'est le
// moteur **Rust** qui fait foi (cf. `finance/`), appelé via IPC. Les formules
// ci-dessous reproduisent fidèlement celles de Rust afin que la prévisualisation
// soit représentative.
import type {
  AmortRow,
  CompoundParams,
  CompoundResult,
  LoanParams,
  LoanResult,
  SavingsParams,
  SavingsResult,
  SeriesPoint,
} from "./ipc";

function monthsCount(years: number): number {
  if (!Number.isFinite(years) || years <= 0) return 0;
  return Math.round(years * 12);
}

export function compound(p: CompoundParams): CompoundResult {
  const n = Math.max(1, Math.trunc(p.compoundsPerYear) || 1);
  const valueAt = (year: number) =>
    p.principal * Math.pow(1 + p.annualRate / n, n * year);

  const finalValue = valueAt(p.years);
  const months = monthsCount(p.years);
  const series: SeriesPoint[] = [];
  for (let k = 0; k <= months; k++) {
    const year = k / 12;
    const value = valueAt(year);
    series.push({ year, value, contributed: p.principal, interest: value - p.principal });
  }
  return {
    finalValue,
    totalContributed: p.principal,
    totalInterest: finalValue - p.principal,
    series,
  };
}

export function savings(p: SavingsParams): SavingsResult {
  const i = p.annualRate / 12;
  const n = monthsCount(p.years);
  const valueAtMonth = (k: number) => {
    if (i === 0) return p.initial + p.monthlyContribution * k;
    const g = Math.pow(1 + i, k);
    let annuity = (p.monthlyContribution * (g - 1)) / i;
    if (p.timing === "start") annuity *= 1 + i;
    return p.initial * g + annuity;
  };

  const series: SeriesPoint[] = [];
  for (let k = 0; k <= n; k++) {
    const value = valueAtMonth(k);
    const contributed = p.initial + p.monthlyContribution * k;
    series.push({ year: k / 12, value, contributed, interest: value - contributed });
  }
  const finalValue = valueAtMonth(n);
  const totalContributed = p.initial + p.monthlyContribution * n;
  return {
    finalValue,
    totalContributed,
    totalInterest: finalValue - totalContributed,
    series,
  };
}

export function loan(p: LoanParams): LoanResult {
  const i = p.annualRate / 12;
  const N = Math.trunc(p.termMonths);
  const prime = (p.principal * p.insuranceAnnualRate) / 12;

  if (N <= 0) {
    // Cas dégénéré : aligné sur le moteur Rust (prime définie, total nul).
    return {
      monthlyPayment: 0,
      monthlyInsurance: prime,
      monthlyTotal: prime,
      totalInterest: 0,
      totalInsurance: 0,
      totalCost: p.principal,
      schedule: [],
    };
  }

  const M = i === 0 ? p.principal / N : (p.principal * i) / (1 - Math.pow(1 + i, -N));
  const schedule: AmortRow[] = [];
  let balance = p.principal;
  let totalInterest = 0;
  for (let m = 1; m <= N; m++) {
    const interestPart = balance * i;
    let principalPart = M - interestPart;
    let payment = M;
    if (m === N) {
      // Dernier mois : solder exactement le capital restant.
      principalPart = balance;
      payment = principalPart + interestPart;
    }
    balance -= principalPart;
    if (m === N) balance = 0;
    totalInterest += interestPart;
    schedule.push({ month: m, payment, principalPart, interestPart, insurance: prime, balance });
  }
  const totalInsurance = prime * N;
  return {
    monthlyPayment: M,
    monthlyInsurance: prime,
    monthlyTotal: M + prime,
    totalInterest,
    totalInsurance,
    totalCost: p.principal + totalInterest + totalInsurance,
    schedule,
  };
}
