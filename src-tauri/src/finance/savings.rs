//! Épargne régulière : capital initial **plus** versements mensuels constants
//! (logique « DCA » — dollar/euro cost averaging).
//!
//! Moteur **pur** (std + serde uniquement) : aucune I/O, aucun état global, donc
//! déterministe et testable contre des valeurs de référence calculées à la main.
//!
//! Hypothèse explicite : **taux mensuel `i = annual_rate / 12`** (nominal) et
//! `N = round(years · 12)` mois. La valeur future suit la formule fermée de
//! l'annuité (cf. [`savings`]).

use super::SeriesPoint;
use serde::{Deserialize, Serialize};

/// Moment du versement dans le mois.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Timing {
    /// Versement en **début** de mois (annuité due) : il génère des intérêts dès
    /// le mois courant.
    Start,
    /// Versement en **fin** de mois (annuité ordinaire) : pas d'intérêt le mois
    /// du versement.
    End,
}

/// Paramètres de l'épargne régulière.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavingsParams {
    /// Capital de départ (peut être 0).
    pub initial: f64,
    /// Versement mensuel constant.
    pub monthly_contribution: f64,
    /// Taux annuel nominal, décimal (5 % => `0.05`).
    pub annual_rate: f64,
    /// Durée, en années (fraction autorisée).
    pub years: f64,
    /// Versement en début ou en fin de mois.
    pub timing: Timing,
}

/// Résultat de l'épargne régulière.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavingsResult {
    /// Valeur finale.
    pub final_value: f64,
    /// Total versé : `initial + monthly_contribution · nb_mois`.
    pub total_contributed: f64,
    /// Intérêts cumulés (`final_value - total_contributed`).
    pub total_interest: f64,
    /// Un point par mois, de `year = 0` à `year = years` inclus.
    pub series: Vec<SeriesPoint>,
}

/// Valeur de l'épargne au mois `k`, pour un facteur mensuel `i` donné.
///
/// - `End`   : `initial·(1+i)^k + PMT·((1+i)^k − 1)/i`
/// - `Start` : idem, terme des versements multiplié par `(1+i)`
/// - `i = 0` : `initial + PMT·k`
///
/// À `k = 0` la valeur vaut toujours `initial` (la somme des versements est vide,
/// quel que soit le timing).
fn value_at_month(initial: f64, pmt: f64, i: f64, k: u32, timing: Timing) -> f64 {
    let k_f = f64::from(k);
    if i == 0.0 {
        return initial + pmt * k_f;
    }
    let growth = (1.0 + i).powf(k_f);
    let annuity = pmt * (growth - 1.0) / i;
    let annuity = match timing {
        Timing::End => annuity,
        Timing::Start => annuity * (1.0 + i),
    };
    initial * growth + annuity
}

/// Calcule la valeur d'une épargne régulière.
///
/// Hypothèse explicite : **taux mensuel `i = annual_rate / 12`** (nominal),
/// `N = round(years · 12)` mois.
///
/// Valeur future (annuité ordinaire / `End`) :
/// `FV = initial·(1+i)^N + PMT·((1+i)^N − 1)/i`.
/// Pour `Start` (annuité due), le terme des versements est multiplié par `(1+i)`.
/// Cas limite `i = 0` : `FV = initial + PMT·N`.
///
/// La `series` contient un point par mois ; `contributed` au mois `k` vaut
/// `initial + PMT·k`.
pub fn savings(params: &SavingsParams) -> SavingsResult {
    let SavingsParams {
        initial,
        monthly_contribution: pmt,
        annual_rate,
        years,
        timing,
    } = *params;

    let i = annual_rate / 12.0;
    let n = super::months_count(years);

    // Un point par mois, de k = 0 à k = N inclus => N + 1 points.
    let mut series = Vec::with_capacity((n as usize).saturating_add(1));
    for k in 0..=n {
        let value = value_at_month(initial, pmt, i, k, timing);
        let contributed = initial + pmt * f64::from(k);
        let year = f64::from(k) / 12.0;
        series.push(SeriesPoint::new(year, value, contributed));
    }

    let final_value = value_at_month(initial, pmt, i, n, timing);
    let total_contributed = initial + pmt * f64::from(n);
    let total_interest = final_value - total_contributed;

    SavingsResult {
        final_value,
        total_contributed,
        total_interest,
        series,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tolérance relative-ish pour les ratios / facteurs sans dimension.
    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-6
    }

    /// Tolérance absolue pour les montants en euros (deux décimales utiles).
    fn approx_money(a: f64, b: f64) -> bool {
        (a - b).abs() < 0.01
    }

    /// Cas taux = 0 : aucune capitalisation, FV = initial + PMT·N.
    ///
    /// initial = 0, PMT = 100, r = 0, t = 1 an => N = 12.
    /// FV = 0 + 100·12 = 1200.00 (timing sans effet quand i = 0).
    /// total_contributed = 1200.00 ; total_interest = 0.00.
    #[test]
    fn taux_zero_donne_somme_des_versements() {
        for timing in [Timing::End, Timing::Start] {
            let res = savings(&SavingsParams {
                initial: 0.0,
                monthly_contribution: 100.0,
                annual_rate: 0.0,
                years: 1.0,
                timing,
            });
            assert!(approx_money(res.final_value, 1200.0), "FV={}", res.final_value);
            assert!(approx_money(res.total_contributed, 1200.0));
            assert!(approx_money(res.total_interest, 0.0));
            // 13 points (k = 0..=12).
            assert_eq!(res.series.len(), 13);
            // k = 0 : value = contributed = initial = 0, interest = 0.
            assert!(approx_money(res.series[0].value, 0.0));
            assert!(approx_money(res.series[0].contributed, 0.0));
            assert!(approx_money(res.series[0].interest, 0.0));
            // k = 6 : value = contributed = 600, interest = 0.
            assert!(approx_money(res.series[6].value, 600.0));
            assert!(approx_money(res.series[6].contributed, 600.0));
            assert!(approx_money(res.series[6].interest, 0.0));
            // Dernier point cohérent avec la valeur finale.
            assert!(approx_money(res.series[12].value, res.final_value));
            assert!(approx_money(res.series[12].year, 1.0));
        }
    }

    /// Cas nominal — annuité ordinaire (End).
    ///
    /// initial = 0, PMT = 100, r = 0.06 => i = 0.005, t = 1 => N = 12.
    /// (1.005)^12 = 1.061677811864498...
    /// FV_End = 100·((1.061677811864498 − 1) / 0.005)
    ///        = 100·(0.061677811864498 / 0.005)
    ///        = 100·12.3355623729   = 1233.5562372900 ≈ 1233.56.
    /// total_contributed = 100·12 = 1200.00.
    /// total_interest = 1233.5562 − 1200 = 33.5562 ≈ 33.56.
    #[test]
    fn nominal_end_annuite_ordinaire() {
        let res = savings(&SavingsParams {
            initial: 0.0,
            monthly_contribution: 100.0,
            annual_rate: 0.06,
            years: 1.0,
            timing: Timing::End,
        });
        assert!(approx_money(res.final_value, 1233.5562373), "FV={}", res.final_value);
        assert!(approx_money(res.total_contributed, 1200.0));
        assert!(approx_money(res.total_interest, 33.5562373));
        assert_eq!(res.series.len(), 13);
        // k = 0 : value = initial = 0, interest = 0.
        assert!(approx_money(res.series[0].value, 0.0));
        assert!(approx_money(res.series[0].interest, 0.0));
        // k = 6 : value = 100·((1.005^6 − 1)/0.005).
        // 1.005^6 = 1.030377509383...  => (0.030377509383/0.005) = 6.0755018766
        // => value = 607.55018766 ≈ 607.55 ; contributed = 600.00.
        assert!(approx_money(res.series[6].value, 607.5501879), "v6={}", res.series[6].value);
        assert!(approx_money(res.series[6].contributed, 600.0));
        assert!(approx_money(res.series[6].interest, 7.5501879));
        // Invariant SeriesPoint : interest == value − contributed sur chaque point.
        for p in &res.series {
            assert!(approx_money(p.interest, p.value - p.contributed));
        }
    }

    /// Cas nominal — annuité due (Start) : même base, terme · (1 + i).
    ///
    /// FV_Start = FV_End · 1.005 = 1233.5562372900 · 1.005 = 1239.7240184765
    ///          ≈ 1239.72.
    /// total_contributed = 1200.00 ; total_interest ≈ 39.72.
    #[test]
    fn nominal_start_annuite_due() {
        let res = savings(&SavingsParams {
            initial: 0.0,
            monthly_contribution: 100.0,
            annual_rate: 0.06,
            years: 1.0,
            timing: Timing::Start,
        });
        assert!(approx_money(res.final_value, 1239.7240185), "FV={}", res.final_value);
        assert!(approx_money(res.total_contributed, 1200.0));
        assert!(approx_money(res.total_interest, 39.7240185));
        // Start = End · (1 + i) sur la valeur finale.
        let end = savings(&SavingsParams {
            initial: 0.0,
            monthly_contribution: 100.0,
            annual_rate: 0.06,
            years: 1.0,
            timing: Timing::End,
        });
        assert!(approx(res.final_value, end.final_value * 1.005));
    }

    /// Cas nominal complet avec capital initial non nul, sur 2 ans.
    ///
    /// initial = 1000, PMT = 200, r = 0.06 => i = 0.005, t = 2 => N = 24.
    /// (1.005)^24 = 1.127159776378...
    /// initial·growth = 1000·1.127159776378 = 1127.159776378.
    /// annuity_End = 200·((1.127159776378 − 1)/0.005)
    ///             = 200·(0.127159776378/0.005) = 200·25.4319552756 = 5086.391055.
    /// FV_End = 1127.159776 + 5086.391055 = 6213.550824 ≈ 6213.55.
    /// FV_Start = 1127.159776 + 5086.391055·1.005 = 6238.98278 ≈ 6238.98.
    /// total_contributed = 1000 + 200·24 = 5800.00.
    #[test]
    fn nominal_avec_initial_sur_deux_ans() {
        let end = savings(&SavingsParams {
            initial: 1000.0,
            monthly_contribution: 200.0,
            annual_rate: 0.06,
            years: 2.0,
            timing: Timing::End,
        });
        assert!(approx_money(end.final_value, 6213.5508244), "FV_End={}", end.final_value);
        assert!(approx_money(end.total_contributed, 5800.0));
        assert!(approx_money(end.total_interest, 413.5508244));
        assert_eq!(end.series.len(), 25);
        // k = 0 : value = initial = 1000, contributed = 1000, interest = 0.
        assert!(approx_money(end.series[0].value, 1000.0));
        assert!(approx_money(end.series[0].contributed, 1000.0));
        assert!(approx_money(end.series[0].interest, 0.0));

        let start = savings(&SavingsParams {
            initial: 1000.0,
            monthly_contribution: 200.0,
            annual_rate: 0.06,
            years: 2.0,
            timing: Timing::Start,
        });
        assert!(approx_money(start.final_value, 6238.9827797), "FV_Start={}", start.final_value);
        assert!(approx_money(start.total_contributed, 5800.0));
        // Le terme des versements (et lui seul) est multiplié par (1 + i) :
        // Start − End = annuity_End · i = 5086.391055 · 0.005 = 25.4319553.
        assert!(approx_money(start.final_value - end.final_value, 25.4319553));
    }

    /// Cas limite : durée nulle => un seul point (k = 0), FV = initial.
    ///
    /// initial = 500, PMT = 100, r = 0.06, t = 0 => N = 0.
    /// FV = 500 ; total_contributed = 500 ; total_interest = 0.
    #[test]
    fn duree_nulle_un_seul_point() {
        let res = savings(&SavingsParams {
            initial: 500.0,
            monthly_contribution: 100.0,
            annual_rate: 0.06,
            years: 0.0,
            timing: Timing::End,
        });
        assert_eq!(res.series.len(), 1);
        assert!(approx_money(res.final_value, 500.0));
        assert!(approx_money(res.total_contributed, 500.0));
        assert!(approx_money(res.total_interest, 0.0));
        assert!(approx_money(res.series[0].value, 500.0));
        assert!(approx_money(res.series[0].year, 0.0));
    }

    /// Cas limite : tout à zéro (initial = 0, PMT = 0) => série plate à zéro.
    #[test]
    fn tout_nul_serie_plate() {
        let res = savings(&SavingsParams {
            initial: 0.0,
            monthly_contribution: 0.0,
            annual_rate: 0.05,
            years: 1.0,
            timing: Timing::Start,
        });
        assert!(approx_money(res.final_value, 0.0));
        assert!(approx_money(res.total_contributed, 0.0));
        assert!(approx_money(res.total_interest, 0.0));
        for p in &res.series {
            assert!(approx_money(p.value, 0.0));
            assert!(approx_money(p.contributed, 0.0));
            assert!(approx_money(p.interest, 0.0));
        }
    }

    /// Arrondi du nombre de mois : N = round(years · 12).
    /// years = 1.5 => 18 mois => 19 points ; years = 0.04 => round(0.48) = 0.
    #[test]
    fn arrondi_nombre_de_mois() {
        let res = savings(&SavingsParams {
            initial: 0.0,
            monthly_contribution: 10.0,
            annual_rate: 0.0,
            years: 1.5,
            timing: Timing::End,
        });
        assert_eq!(res.series.len(), 19); // k = 0..=18
        assert!(approx_money(res.total_contributed, 180.0));

        let res0 = savings(&SavingsParams {
            initial: 100.0,
            monthly_contribution: 10.0,
            annual_rate: 0.0,
            years: 0.04, // 0.04 · 12 = 0.48 => round = 0
            timing: Timing::End,
        });
        assert_eq!(res0.series.len(), 1);
        assert!(approx_money(res0.final_value, 100.0));
    }
}
