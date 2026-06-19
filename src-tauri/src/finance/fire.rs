//! Indépendance financière (FIRE) : à partir d'un capital de départ, de
//! versements mensuels et d'un rendement, calcule le **capital cible**
//! (« FI number » = dépenses annuelles ÷ taux de retrait) et le **temps**
//! nécessaire pour l'atteindre.
//!
//! Moteur **pur** (std + serde) ; testé contre des valeurs de référence.

use super::SeriesPoint;
use serde::{Deserialize, Serialize};

/// Paramètres FIRE.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FireParams {
    pub initial_capital: f64,
    pub monthly_contribution: f64,
    /// Taux annuel nominal, décimal (5 % => 0.05). Taux mensuel = r/12.
    pub annual_rate: f64,
    /// Dépenses annuelles visées à l'indépendance.
    pub annual_expenses: f64,
    /// Taux de retrait sécurisé (ex. 0.04 = règle des 4 %).
    pub withdrawal_rate: f64,
}

/// Résultat FIRE.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FireResult {
    /// Capital cible = `annual_expenses / withdrawal_rate`.
    pub fi_number: f64,
    /// Nombre de mois pour atteindre la cible (None si hors d'atteinte).
    pub months_to_fi: Option<u32>,
    /// Années pour atteindre la cible (None si hors d'atteinte).
    pub years_to_fi: Option<f64>,
    /// Capital à la fin de l'horizon de la série.
    pub final_capital: f64,
    pub target_reached: bool,
    /// Un point par mois (capital vs versé) jusqu'à FI (ou horizon borné).
    pub series: Vec<SeriesPoint>,
}

/// Capital accumulé au mois `k` (versements en fin de mois).
fn capital_at(initial: f64, monthly: f64, i: f64, k: u32) -> f64 {
    let k_f = f64::from(k);
    if i == 0.0 {
        initial + monthly * k_f
    } else {
        let growth = (1.0 + i).powf(k_f);
        initial * growth + monthly * (growth - 1.0) / i
    }
}

/// Calcule l'indépendance financière.
pub fn fire(params: &FireParams) -> FireResult {
    let i = params.annual_rate / 12.0;
    let fi_number = if params.withdrawal_rate > 0.0 {
        params.annual_expenses / params.withdrawal_rate
    } else {
        f64::INFINITY
    };
    let max_months = super::MAX_PROJECTION_MONTHS;

    let mut months_to_fi: Option<u32> = None;
    if fi_number.is_finite() {
        for k in 0..=max_months {
            if capital_at(params.initial_capital, params.monthly_contribution, i, k) >= fi_number {
                months_to_fi = Some(k);
                break;
            }
        }
    }
    let target_reached = months_to_fi.is_some();

    // Horizon de la série : jusqu'à l'année qui suit FI, sinon 50 ans, borné.
    let horizon_months = match months_to_fi {
        Some(m) => ((m / 12) + 1) * 12,
        None => 50 * 12,
    }
    .clamp(12, max_months);

    let mut series = Vec::with_capacity(horizon_months as usize + 1);
    for k in 0..=horizon_months {
        let value = capital_at(params.initial_capital, params.monthly_contribution, i, k);
        let contributed = params.initial_capital + params.monthly_contribution * f64::from(k);
        series.push(SeriesPoint::new(f64::from(k) / 12.0, value, contributed));
    }

    let final_capital =
        capital_at(params.initial_capital, params.monthly_contribution, i, horizon_months);
    let years_to_fi = months_to_fi.map(|m| f64::from(m) / 12.0);

    FireResult {
        fi_number,
        months_to_fi,
        years_to_fi,
        final_capital,
        target_reached,
        series,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 0.01
    }

    /// FI number = dépenses / taux de retrait.
    /// Dépenses 20000, retrait 4 % => cible 500 000.
    #[test]
    fn fi_number_basic() {
        let r = fire(&FireParams {
            initial_capital: 0.0,
            monthly_contribution: 1000.0,
            annual_rate: 0.05,
            annual_expenses: 20_000.0,
            withdrawal_rate: 0.04,
        });
        assert!(approx(r.fi_number, 500_000.0));
        assert!(r.target_reached);
        let m = r.months_to_fi.unwrap();
        // Le mois trouvé franchit la cible, le mois précédent non.
        assert!(capital_at(0.0, 1000.0, 0.05 / 12.0, m) >= r.fi_number);
        assert!(m == 0 || capital_at(0.0, 1000.0, 0.05 / 12.0, m - 1) < r.fi_number);
    }

    /// Taux de rendement nul : capital = versements cumulés ; franchissement exact.
    /// dépenses 12000, retrait 4 % => cible 300 000 ; 1000/mois => 300 mois (25 ans).
    #[test]
    fn zero_rate_exact_crossing() {
        let r = fire(&FireParams {
            initial_capital: 0.0,
            monthly_contribution: 1000.0,
            annual_rate: 0.0,
            annual_expenses: 12_000.0,
            withdrawal_rate: 0.04,
        });
        assert!(approx(r.fi_number, 300_000.0));
        assert_eq!(r.months_to_fi, Some(300));
        assert!(approx(r.years_to_fi.unwrap(), 25.0));
    }

    /// Capital déjà suffisant : FI atteinte au mois 0.
    #[test]
    fn already_independent() {
        let r = fire(&FireParams {
            initial_capital: 600_000.0,
            monthly_contribution: 0.0,
            annual_rate: 0.05,
            annual_expenses: 20_000.0,
            withdrawal_rate: 0.04,
        });
        assert_eq!(r.months_to_fi, Some(0));
        assert!(approx(r.years_to_fi.unwrap(), 0.0));
    }

    /// Taux de retrait nul : cible infinie, jamais atteinte.
    #[test]
    fn zero_withdrawal_rate_never() {
        let r = fire(&FireParams {
            initial_capital: 1000.0,
            monthly_contribution: 100.0,
            annual_rate: 0.05,
            annual_expenses: 20_000.0,
            withdrawal_rate: 0.0,
        });
        assert!(!r.target_reached);
        assert_eq!(r.months_to_fi, None);
        assert!(r.fi_number.is_infinite());
    }
}
