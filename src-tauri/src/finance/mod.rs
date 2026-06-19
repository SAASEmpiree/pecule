//! Moteurs de calcul financier de Pécule.
//!
//! Tous les moteurs sont **purs** : aucune dépendance à Tauri, aucune I/O,
//! aucun état global. Cela les rend déterministes et testables contre des
//! **valeurs de référence** (cf. §24 du cahier de conception).
//!
//! Conventions partagées par tous les moteurs :
//! - Les **taux** sont annuels et **décimaux** : 5 % s'écrit `0.05`.
//! - Les durées « longues » sont en **années** (`f64`, fraction autorisée) ;
//!   les durées de prêt sont en **mois** (`u32`).
//! - Les **montants** sont des `f64` en unité monétaire (euros). L'arrondi à
//!   deux décimales se fait à l'affichage, jamais dans les moteurs.
//! - Chaque moteur renvoie une **série temporelle** (`Vec<SeriesPoint>`) prête
//!   à être tracée par « la courbe » : un point par mois, borne 0 incluse.

use serde::{Deserialize, Serialize};

pub mod compound;
pub mod fire;
pub mod loan;
pub mod savings;

/// Un point d'une projection temporelle, prêt à être tracé par « la courbe ».
///
/// Invariant : `interest == value - contributed` (aux erreurs d'arrondi près).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeriesPoint {
    /// Temps écoulé depuis aujourd'hui, en **années** (0.0 = aujourd'hui).
    pub year: f64,
    /// Valeur totale du capital à cet instant.
    pub value: f64,
    /// Total des sommes **versées** (capital injecté) jusqu'à cet instant.
    pub contributed: f64,
    /// Intérêts cumulés à cet instant (`value - contributed`).
    pub interest: f64,
}

impl SeriesPoint {
    /// Construit un point en déduisant les intérêts de `value - contributed`.
    pub fn new(year: f64, value: f64, contributed: f64) -> Self {
        SeriesPoint {
            year,
            value,
            contributed,
            interest: value - contributed,
        }
    }
}

/// Plafond métier de projection : **200 ans** (2400 mois). Borne l'allocation
/// des séries temporelles et neutralise toute durée non finie ou démesurée.
pub const MAX_PROJECTION_MONTHS: u32 = 200 * 12;

/// Nombre de mois à projeter pour une durée en années : `round(years · 12)`,
/// avec plancher 0 et plafond [`MAX_PROJECTION_MONTHS`]. Une durée non finie
/// (NaN, ±∞) ou ≤ 0 donne 0.
///
/// Mutualisé par tous les moteurs : garantit une allocation bornée (jamais de
/// `Vec` géant) et un comportement identique face aux entrées aberrantes.
pub fn months_count(years: f64) -> u32 {
    if !years.is_finite() || years <= 0.0 {
        return 0;
    }
    let n = (years * 12.0).round();
    if n >= f64::from(MAX_PROJECTION_MONTHS) {
        MAX_PROJECTION_MONTHS
    } else {
        // `n` est fini, positif et < MAX : le cast est exact.
        n as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn months_count_is_bounded_and_safe() {
        assert_eq!(months_count(0.0), 0);
        assert_eq!(months_count(-5.0), 0);
        assert_eq!(months_count(f64::NAN), 0);
        assert_eq!(months_count(f64::INFINITY), 0);
        assert_eq!(months_count(10.0), 120);
        assert_eq!(months_count(1.5), 18);
        // Durées absurdes : plafonnées à 200 ans, jamais d'allocation géante.
        assert_eq!(months_count(500.0), MAX_PROJECTION_MONTHS);
        assert_eq!(months_count(1.0e9), MAX_PROJECTION_MONTHS);
    }
}
