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
