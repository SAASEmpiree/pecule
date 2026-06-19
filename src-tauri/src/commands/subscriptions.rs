//! Commandes des **abonnements** (v0.3) : CRUD sur la table `subscriptions`,
//! persistées en SQLite local. Le coût mensuel/annuel réel et les échéances
//! sont calculés côté front à partir de `montant` + `frequence`.

use crate::db::Db;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use tauri::State;

/// Un abonnement.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
    pub id: i64,
    pub nom: String,
    pub categorie: Option<String>,
    pub montant: f64,
    /// "mensuel" | "annuel" | "trimestriel" | "hebdomadaire".
    pub frequence: String,
    /// Date ISO 8601 du prochain prélèvement (AAAA-MM-JJ), optionnelle.
    pub prochain_prelevement: Option<String>,
    pub actif: bool,
}

/// Données pour créer un abonnement (sans `id`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewSubscription {
    pub nom: String,
    pub categorie: Option<String>,
    pub montant: f64,
    pub frequence: String,
    pub prochain_prelevement: Option<String>,
    pub actif: bool,
}

fn q_list(conn: &Connection) -> rusqlite::Result<Vec<Subscription>> {
    let mut stmt = conn.prepare(
        "SELECT id, nom, categorie, montant, frequence, prochain_prelevement, actif
         FROM subscriptions ORDER BY id",
    )?;
    let rows = stmt.query_map([], |r| {
        Ok(Subscription {
            id: r.get(0)?,
            nom: r.get(1)?,
            categorie: r.get(2)?,
            montant: r.get(3)?,
            frequence: r.get(4)?,
            prochain_prelevement: r.get(5)?,
            actif: r.get(6)?,
        })
    })?;
    rows.collect()
}

fn q_insert(conn: &Connection, s: &NewSubscription) -> rusqlite::Result<i64> {
    conn.execute(
        "INSERT INTO subscriptions (nom, categorie, montant, frequence, prochain_prelevement, actif)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![s.nom, s.categorie, s.montant, s.frequence, s.prochain_prelevement, s.actif],
    )?;
    Ok(conn.last_insert_rowid())
}

fn q_update(conn: &Connection, s: &Subscription) -> rusqlite::Result<usize> {
    conn.execute(
        "UPDATE subscriptions
         SET nom=?2, categorie=?3, montant=?4, frequence=?5, prochain_prelevement=?6, actif=?7
         WHERE id=?1",
        params![
            s.id,
            s.nom,
            s.categorie,
            s.montant,
            s.frequence,
            s.prochain_prelevement,
            s.actif
        ],
    )
}

fn q_delete(conn: &Connection, id: i64) -> rusqlite::Result<usize> {
    conn.execute("DELETE FROM subscriptions WHERE id=?1", params![id])
}

#[tauri::command]
pub fn list_subscriptions(db: State<Db>) -> Result<Vec<Subscription>, String> {
    let conn = db.0.lock().unwrap_or_else(|p| p.into_inner());
    q_list(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn add_subscription(db: State<Db>, subscription: NewSubscription) -> Result<i64, String> {
    let conn = db.0.lock().unwrap_or_else(|p| p.into_inner());
    q_insert(&conn, &subscription).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_subscription(db: State<Db>, subscription: Subscription) -> Result<(), String> {
    let conn = db.0.lock().unwrap_or_else(|p| p.into_inner());
    q_update(&conn, &subscription).map(|_| ()).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_subscription(db: State<Db>, id: i64) -> Result<(), String> {
    let conn = db.0.lock().unwrap_or_else(|p| p.into_inner());
    q_delete(&conn, id).map(|_| ()).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crud_roundtrip() {
        let mut path = std::env::temp_dir();
        path.push("pecule_subs_test.sqlite");
        let _ = std::fs::remove_file(&path);
        let db = Db::open(&path).expect("ouverture base");
        let conn = db.0.lock().unwrap();

        assert!(q_list(&conn).unwrap().is_empty());

        let id = q_insert(
            &conn,
            &NewSubscription {
                nom: "Spotify".into(),
                categorie: Some("Musique".into()),
                montant: 10.99,
                frequence: "mensuel".into(),
                prochain_prelevement: Some("2026-07-01".into()),
                actif: true,
            },
        )
        .unwrap();
        assert!(id > 0);

        let list = q_list(&conn).unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].nom, "Spotify");
        assert_eq!(list[0].categorie.as_deref(), Some("Musique"));
        assert!(list[0].actif);

        let mut s = list[0].clone();
        s.montant = 11.99;
        s.actif = false;
        assert_eq!(q_update(&conn, &s).unwrap(), 1);
        let updated = q_list(&conn).unwrap();
        assert!((updated[0].montant - 11.99).abs() < 1e-9);
        assert!(!updated[0].actif);

        assert_eq!(q_delete(&conn, id).unwrap(), 1);
        assert!(q_list(&conn).unwrap().is_empty());

        drop(conn);
        let _ = std::fs::remove_file(&path);
    }
}
