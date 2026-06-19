//! Commandes de simulation : fines enveloppes IPC autour des moteurs purs de
//! `finance`. Elles renvoient la **série complète** afin que le curseur « et dans
//! X années ? » de la courbe puisse être déplacé sans nouvel appel réseau/IPC.
//!
//! Les entrées venant du frontend sont **assainies** ici (frontière IPC) : toute
//! valeur non finie (NaN, ±∞) ou hors plage est ramenée à une valeur sûre, afin
//! qu'aucune saisie aberrante ne produise un graphe cassé ou une allocation
//! démesurée côté moteur.

use crate::finance::compound::{compound, CompoundParams, CompoundResult};
use crate::finance::fire::{fire, FireParams, FireResult};
use crate::finance::loan::{loan, LoanParams, LoanResult};
use crate::finance::savings::{savings, SavingsParams, SavingsResult};

/// Rend une valeur finie : remplace NaN/±∞ par `default`.
fn finite(x: f64, default: f64) -> f64 {
    if x.is_finite() {
        x
    } else {
        default
    }
}

/// Montant assaini : fini, positif ou nul.
fn amount(x: f64) -> f64 {
    finite(x, 0.0).max(0.0)
}

/// Durée en années assainie : finie, bornée à [0, 200].
fn years(x: f64) -> f64 {
    finite(x, 0.0).clamp(0.0, 200.0)
}

/// Taux annuel assaini : fini, borné à une plage saine. Le plancher `-0.95`
/// garantit une base `1 + r/n > 0` (avec `n ≥ 1`), donc jamais de `NaN` via `powf`.
fn rate(x: f64) -> f64 {
    finite(x, 0.0).clamp(-0.95, 100.0)
}

#[tauri::command]
pub fn simulate_compound(mut params: CompoundParams) -> CompoundResult {
    params.principal = amount(params.principal);
    params.annual_rate = rate(params.annual_rate);
    params.years = years(params.years);
    compound(&params)
}

#[tauri::command]
pub fn simulate_savings(mut params: SavingsParams) -> SavingsResult {
    params.initial = amount(params.initial);
    params.monthly_contribution = amount(params.monthly_contribution);
    params.annual_rate = rate(params.annual_rate);
    params.years = years(params.years);
    savings(&params)
}

#[tauri::command]
pub fn simulate_loan(mut params: LoanParams) -> LoanResult {
    params.principal = amount(params.principal);
    params.annual_rate = rate(params.annual_rate);
    params.insurance_annual_rate = amount(params.insurance_annual_rate);
    // `term_months` est borné par le moteur (cf. super::MAX_PROJECTION_MONTHS).
    loan(&params)
}

#[tauri::command]
pub fn simulate_fire(mut params: FireParams) -> FireResult {
    params.initial_capital = amount(params.initial_capital);
    params.monthly_contribution = amount(params.monthly_contribution);
    params.annual_rate = rate(params.annual_rate);
    params.annual_expenses = amount(params.annual_expenses);
    params.withdrawal_rate = finite(params.withdrawal_rate, 0.0).clamp(0.0, 1.0);
    fire(&params)
}
