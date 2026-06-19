//! Intérêt composé : un capital unique placé, **sans** versement périodique.
//!
//! Statut : **implémenté**. Conserver tels quels les types publics et la
//! signature de la fonction (la couche `commands` en dépend).

use super::SeriesPoint;
use serde::{Deserialize, Serialize};

/// Paramètres de l'intérêt composé.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompoundParams {
    /// Capital initial placé.
    pub principal: f64,
    /// Taux annuel nominal, décimal (5 % => `0.05`).
    pub annual_rate: f64,
    /// Durée du placement, en années (fraction autorisée).
    pub years: f64,
    /// Nombre de capitalisations par an (1 = annuel, 12 = mensuel, …). Min 1.
    pub compounds_per_year: u32,
}

/// Résultat de l'intérêt composé.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompoundResult {
    /// Valeur finale `FV` à `t = years`.
    pub final_value: f64,
    /// Total versé (== `principal`, aucun versement périodique).
    pub total_contributed: f64,
    /// Intérêts cumulés (`final_value - principal`).
    pub total_interest: f64,
    /// Un point par mois, de `year = 0` à `year = years` inclus.
    pub series: Vec<SeriesPoint>,
}

/// Calcule l'intérêt composé d'un capital unique.
///
/// Formule fermée : `FV = P · (1 + r/n)^(n·t)` avec
/// `r = annual_rate`, `n = compounds_per_year`, `t = years`, `P = principal`.
///
/// La `series` contient un point par **mois** (`year = k/12`, `k = 0..=12·years`),
/// chaque valeur calculée par la même formule à l'instant `year` pour une courbe
/// lisse ; `contributed` y est constant et vaut `principal`.
pub fn compound(params: &CompoundParams) -> CompoundResult {
    let principal = params.principal;
    // `compounds_per_year` est borné à au moins 1 pour éviter toute division
    // par zéro (cas `compounds_per_year == 0` traité comme annuel).
    let n = params.compounds_per_year.max(1);

    // Valeur du capital à l'instant `year` (en années), formule fermée appliquée
    // au temps fractionnaire pour produire une courbe lisse.
    let value_at = |year: f64| future_value(principal, params.annual_rate, n, year);

    // Nombre de mois à projeter (borné et sûr, cf. `super::months_count`).
    let months = super::months_count(params.years);

    // `final_value` est calculé au dernier instant tracé (`months / 12`) afin
    // d'être strictement cohérent avec l'extrémité droite de la courbe : aucun
    // écart entre le chiffre « valeur finale » et le bout du graphe.
    let last_year = f64::from(months) / 12.0;
    let final_value = value_at(last_year);

    let mut series = Vec::with_capacity(months as usize + 1);
    for k in 0..=months {
        let year = f64::from(k) / 12.0;
        let value = value_at(year);
        // `contributed` est constant : il n'y a aucun versement périodique.
        series.push(SeriesPoint::new(year, value, principal));
    }

    CompoundResult {
        final_value,
        total_contributed: principal,
        total_interest: final_value - principal,
        series,
    }
}

/// `FV = P · (1 + r/n)^(n·t)`. Pour `r = 0`, le facteur vaut `1` donc `FV = P`.
fn future_value(principal: f64, annual_rate: f64, n: u32, years: f64) -> f64 {
    let n_f = f64::from(n);
    let base = 1.0 + annual_rate / n_f;
    let exponent = n_f * years;
    principal * base.powf(exponent)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tolérance relative pour les ratios / facteurs sans dimension.
    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-6
    }

    /// Tolérance absolue pour les montants exprimés en euros.
    fn approx_money(a: f64, b: f64) -> bool {
        (a - b).abs() < 0.01
    }

    /// Cas nominal annuel : P=1000, r=0.05, t=10, n=1.
    /// FV = 1000 · 1.05^10.
    /// 1.05^10 = 1.628894626777442 → FV ≈ 1628.894626777442 €.
    /// total_interest = FV - 1000 ≈ 628.894626777442 €.
    #[test]
    fn nominal_annual() {
        let params = CompoundParams {
            principal: 1000.0,
            annual_rate: 0.05,
            years: 10.0,
            compounds_per_year: 1,
        };
        let r = compound(&params);

        assert!(approx_money(r.final_value, 1628.894626777442));
        assert!(approx_money(r.total_contributed, 1000.0));
        assert!(approx_money(r.total_interest, 628.894626777442));

        // Série : un point par mois de year=0 à year=10 → 121 points (0..=120).
        assert_eq!(r.series.len(), 121);

        // Premier point : year=0 → value = P = 1000, intérêts = 0.
        let p0 = r.series[0];
        assert!(approx(p0.year, 0.0));
        assert!(approx_money(p0.value, 1000.0));
        assert!(approx_money(p0.contributed, 1000.0));
        assert!(approx_money(p0.interest, 0.0));

        // Point à year=1 (k=12) : value = 1000 · 1.05^1 = 1050.0.
        let p12 = r.series[12];
        assert!(approx(p12.year, 1.0));
        assert!(approx_money(p12.value, 1050.0));
        assert!(approx_money(p12.contributed, 1000.0));
        assert!(approx_money(p12.interest, 50.0));

        // Dernier point : year=10 (k=120) → value = final_value.
        let last = r.series[120];
        assert!(approx(last.year, 10.0));
        assert!(approx_money(last.value, r.final_value));
        // Invariant SeriesPoint : interest == value - contributed.
        assert!(approx_money(last.interest, last.value - last.contributed));
    }

    /// Cas nominal mensuel : P=1000, r=0.05, t=10, n=12.
    /// FV = 1000 · (1 + 0.05/12)^120.
    /// (1 + 0.0041666…)^120 = 1.6470094976902801 → FV ≈ 1647.00949769028 €.
    /// Point à year=1/12 (k=1) : 1000 · (1+0.05/12)^(12·(1/12))
    ///   = 1000 · (1.0041666…)^1 = 1004.1666666666666 €.
    #[test]
    fn nominal_monthly() {
        let params = CompoundParams {
            principal: 1000.0,
            annual_rate: 0.05,
            years: 10.0,
            compounds_per_year: 12,
        };
        let r = compound(&params);

        assert!(approx_money(r.final_value, 1647.00949769028));
        assert!(approx_money(r.total_contributed, 1000.0));
        assert!(approx_money(r.total_interest, 647.0094976902801));

        // k=1 → year = 1/12, value = 1000 · (1+0.05/12) = 1004.1666666666666.
        let p1 = r.series[1];
        assert!(approx(p1.year, 1.0 / 12.0));
        assert!(approx_money(p1.value, 1004.1666666666666));
    }

    /// Capitalisation semestrielle (cas limite « n ni 1 ni 12 »).
    /// P=1000, r=0.10, t=3, n=2 → FV = 1000 · (1+0.10/2)^(2·3)
    ///   = 1000 · 1.05^6 = 1340.0956406250004 €.
    #[test]
    fn semiannual_compounding() {
        let params = CompoundParams {
            principal: 1000.0,
            annual_rate: 0.10,
            years: 3.0,
            compounds_per_year: 2,
        };
        let r = compound(&params);

        assert!(approx_money(r.final_value, 1340.0956406250004));
        assert!(approx_money(r.total_interest, 340.0956406250004));
    }

    /// Cas taux = 0 : série plate à P, aucun intérêt.
    /// FV = P · (1 + 0)^(n·t) = P quel que soit n et t.
    #[test]
    fn zero_rate_is_flat() {
        let params = CompoundParams {
            principal: 5000.0,
            annual_rate: 0.0,
            years: 7.0,
            compounds_per_year: 12,
        };
        let r = compound(&params);

        assert!(approx_money(r.final_value, 5000.0));
        assert!(approx_money(r.total_contributed, 5000.0));
        assert!(approx_money(r.total_interest, 0.0));

        // Série plate : 7 ans → 85 points (0..=84), tous à 5000 €.
        assert_eq!(r.series.len(), 85);
        for point in &r.series {
            assert!(approx_money(point.value, 5000.0));
            assert!(approx_money(point.contributed, 5000.0));
            assert!(approx_money(point.interest, 0.0));
        }
    }

    /// Cas years = 0 : un seul point [year=0, value=P, contributed=P, interest=0].
    #[test]
    fn zero_years_single_point() {
        let params = CompoundParams {
            principal: 2500.0,
            annual_rate: 0.05,
            years: 0.0,
            compounds_per_year: 12,
        };
        let r = compound(&params);

        assert!(approx_money(r.final_value, 2500.0));
        assert!(approx_money(r.total_interest, 0.0));

        assert_eq!(r.series.len(), 1);
        let only = r.series[0];
        assert!(approx(only.year, 0.0));
        assert!(approx_money(only.value, 2500.0));
        assert!(approx_money(only.contributed, 2500.0));
        assert!(approx_money(only.interest, 0.0));
    }

    /// Cas compounds_per_year = 0 : doit être traité comme n = 1 (annuel).
    /// FV doit alors être identique au cas n=1 : 1000 · 1.05^10 ≈ 1628.8946… €.
    #[test]
    fn zero_compounds_treated_as_annual() {
        let zero = CompoundParams {
            principal: 1000.0,
            annual_rate: 0.05,
            years: 10.0,
            compounds_per_year: 0,
        };
        let one = CompoundParams {
            compounds_per_year: 1,
            ..zero
        };

        let r_zero = compound(&zero);
        let r_one = compound(&one);

        assert!(approx_money(r_zero.final_value, 1628.894626777442));
        // Équivalence stricte (à l'arrondi près) avec n = 1.
        assert!(approx_money(r_zero.final_value, r_one.final_value));
        assert_eq!(r_zero.series.len(), r_one.series.len());
    }

    /// Cas principal = 0 : tout reste à 0, série plate, pas de NaN.
    #[test]
    fn zero_principal() {
        let params = CompoundParams {
            principal: 0.0,
            annual_rate: 0.05,
            years: 5.0,
            compounds_per_year: 12,
        };
        let r = compound(&params);

        assert!(approx_money(r.final_value, 0.0));
        assert!(approx_money(r.total_contributed, 0.0));
        assert!(approx_money(r.total_interest, 0.0));
        for point in &r.series {
            assert!(approx_money(point.value, 0.0));
        }
    }

    /// Durée fractionnaire : round(years·12) pour le nombre de mois.
    /// years = 1.5 → round(18) = 18 mois → 19 points (0..=18).
    /// Dernier point year=1.5 ; final_value = 1000·(1+0.05/12)^(12·1.5)
    ///   = 1000·(1.0041666…)^18 = 1077.7162109449166 €.
    #[test]
    fn fractional_years_point_count() {
        let params = CompoundParams {
            principal: 1000.0,
            annual_rate: 0.05,
            years: 1.5,
            compounds_per_year: 12,
        };
        let r = compound(&params);

        assert_eq!(r.series.len(), 19);
        assert!(approx_money(r.final_value, 1077.7162109449166));
        let last = r.series[18];
        assert!(approx(last.year, 1.5));
        assert!(approx_money(last.value, r.final_value));
    }
}
