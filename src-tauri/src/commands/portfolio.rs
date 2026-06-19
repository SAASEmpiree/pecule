//! Commandes du **portefeuille** (v0.2) : CRUD des positions sur la table
//! `holdings`, persistées dans la base SQLite locale. Valorisation **manuelle**
//! (le prix actuel est saisi par l'utilisateur ; le rafraîchissement de cours
//! publics reste une option de v2, cf. §11).
//!
//! La logique SQL vit dans des fonctions libres `q_*` prenant `&Connection`,
//! afin d'être testables sans dépendance à l'état Tauri ; les commandes
//! `#[tauri::command]` se contentent de verrouiller la connexion et d'appeler.

use crate::db::Db;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use tauri::State;

/// Compte par défaut (le multi-compte arrive en v2 — cf. §8).
const ACCOUNT_ID: i64 = 1;

/// Une position du portefeuille.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Holding {
    pub id: i64,
    pub nom: String,
    pub symbole: String,
    /// Classe d'actif : "action" | "etf" | "crypto" | "autre".
    #[serde(rename = "type")]
    pub kind: String,
    pub quantite: f64,
    /// Prix de revient unitaire (PRU).
    pub pru: f64,
    /// Prix actuel unitaire (saisi manuellement).
    pub prix_actuel: f64,
    pub devise: String,
}

/// Données pour créer une position (sans `id`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewHolding {
    pub nom: String,
    pub symbole: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub quantite: f64,
    pub pru: f64,
    pub prix_actuel: f64,
    pub devise: String,
}

fn q_list(conn: &Connection) -> rusqlite::Result<Vec<Holding>> {
    let mut stmt = conn.prepare(
        "SELECT id, nom, symbole, type, quantite, pru, prix_actuel, devise
         FROM holdings ORDER BY id",
    )?;
    let rows = stmt.query_map([], |r| {
        Ok(Holding {
            id: r.get(0)?,
            nom: r.get(1)?,
            symbole: r.get(2)?,
            kind: r.get(3)?,
            quantite: r.get(4)?,
            pru: r.get(5)?,
            prix_actuel: r.get(6)?,
            devise: r.get(7)?,
        })
    })?;
    rows.collect()
}

fn q_insert(conn: &Connection, h: &NewHolding) -> rusqlite::Result<i64> {
    conn.execute(
        "INSERT INTO holdings (account_id, nom, symbole, type, quantite, pru, prix_actuel, devise)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            ACCOUNT_ID,
            h.nom,
            h.symbole,
            h.kind,
            h.quantite,
            h.pru,
            h.prix_actuel,
            h.devise
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

fn q_update(conn: &Connection, h: &Holding) -> rusqlite::Result<usize> {
    conn.execute(
        "UPDATE holdings
         SET nom=?2, symbole=?3, type=?4, quantite=?5, pru=?6, prix_actuel=?7, devise=?8
         WHERE id=?1",
        params![
            h.id, h.nom, h.symbole, h.kind, h.quantite, h.pru, h.prix_actuel, h.devise
        ],
    )
}

fn q_delete(conn: &Connection, id: i64) -> rusqlite::Result<usize> {
    conn.execute("DELETE FROM holdings WHERE id=?1", params![id])
}

#[tauri::command]
pub fn list_holdings(db: State<Db>) -> Result<Vec<Holding>, String> {
    let conn = db.0.lock().unwrap_or_else(|p| p.into_inner());
    q_list(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn add_holding(db: State<Db>, holding: NewHolding) -> Result<i64, String> {
    let conn = db.0.lock().unwrap_or_else(|p| p.into_inner());
    q_insert(&conn, &holding).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_holding(db: State<Db>, holding: Holding) -> Result<(), String> {
    let conn = db.0.lock().unwrap_or_else(|p| p.into_inner());
    q_update(&conn, &holding).map(|_| ()).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_holding(db: State<Db>, id: i64) -> Result<(), String> {
    let conn = db.0.lock().unwrap_or_else(|p| p.into_inner());
    q_delete(&conn, id).map(|_| ()).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> NewHolding {
        NewHolding {
            nom: "Air Liquide".into(),
            symbole: "AI.PA".into(),
            kind: "action".into(),
            quantite: 10.0,
            pru: 150.0,
            prix_actuel: 165.0,
            devise: "EUR".into(),
        }
    }

    #[test]
    fn crud_roundtrip() {
        let mut path = std::env::temp_dir();
        path.push("pecule_portfolio_test.sqlite");
        let _ = std::fs::remove_file(&path);
        let db = Db::open(&path).expect("ouverture base");
        let conn = db.0.lock().unwrap();

        // Vide au départ.
        assert!(q_list(&conn).unwrap().is_empty());

        // Insertion.
        let id = q_insert(&conn, &sample()).unwrap();
        assert!(id > 0);
        let list = q_list(&conn).unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].symbole, "AI.PA");
        assert_eq!(list[0].kind, "action");
        assert!((list[0].prix_actuel - 165.0).abs() < 1e-9);

        // Mise à jour.
        let mut h = list[0].clone();
        h.prix_actuel = 170.0;
        h.quantite = 12.0;
        assert_eq!(q_update(&conn, &h).unwrap(), 1);
        let updated = q_list(&conn).unwrap();
        assert!((updated[0].prix_actuel - 170.0).abs() < 1e-9);
        assert!((updated[0].quantite - 12.0).abs() < 1e-9);

        // Suppression.
        assert_eq!(q_delete(&conn, id).unwrap(), 1);
        assert!(q_list(&conn).unwrap().is_empty());

        drop(conn);
        let _ = std::fs::remove_file(&path);
    }
}
