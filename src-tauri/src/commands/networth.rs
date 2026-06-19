//! Commandes du **patrimoine net** (v0.4) : CRUD des actifs/passifs saisis
//! manuellement (comptes, épargne, immobilier, dettes…), hors portefeuille
//! titres. Le patrimoine net se calcule côté front : valeur du portefeuille
//! (positions) + somme des actifs − somme des dettes.

use crate::db::Db;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use tauri::State;

/// Un actif (ou passif si `est_dette`) du patrimoine.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub id: i64,
    pub nom: String,
    /// "compte" | "epargne" | "immobilier" | "autre" | "dette".
    #[serde(rename = "type")]
    pub kind: String,
    pub valeur: f64,
    pub est_dette: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewAsset {
    pub nom: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub valeur: f64,
    pub est_dette: bool,
}

fn q_list(conn: &Connection) -> rusqlite::Result<Vec<Asset>> {
    let mut stmt =
        conn.prepare("SELECT id, nom, type, valeur, est_dette FROM assets ORDER BY id")?;
    let rows = stmt.query_map([], |r| {
        Ok(Asset {
            id: r.get(0)?,
            nom: r.get(1)?,
            kind: r.get(2)?,
            valeur: r.get(3)?,
            est_dette: r.get(4)?,
        })
    })?;
    rows.collect()
}

fn q_insert(conn: &Connection, a: &NewAsset) -> rusqlite::Result<i64> {
    conn.execute(
        "INSERT INTO assets (nom, type, valeur, est_dette) VALUES (?1, ?2, ?3, ?4)",
        params![a.nom, a.kind, a.valeur, a.est_dette],
    )?;
    Ok(conn.last_insert_rowid())
}

fn q_update(conn: &Connection, a: &Asset) -> rusqlite::Result<usize> {
    conn.execute(
        "UPDATE assets SET nom=?2, type=?3, valeur=?4, est_dette=?5 WHERE id=?1",
        params![a.id, a.nom, a.kind, a.valeur, a.est_dette],
    )
}

fn q_delete(conn: &Connection, id: i64) -> rusqlite::Result<usize> {
    conn.execute("DELETE FROM assets WHERE id=?1", params![id])
}

#[tauri::command]
pub fn list_assets(db: State<Db>) -> Result<Vec<Asset>, String> {
    let conn = db.0.lock().unwrap_or_else(|p| p.into_inner());
    q_list(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn add_asset(db: State<Db>, asset: NewAsset) -> Result<i64, String> {
    let conn = db.0.lock().unwrap_or_else(|p| p.into_inner());
    q_insert(&conn, &asset).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_asset(db: State<Db>, asset: Asset) -> Result<(), String> {
    let conn = db.0.lock().unwrap_or_else(|p| p.into_inner());
    q_update(&conn, &asset).map(|_| ()).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_asset(db: State<Db>, id: i64) -> Result<(), String> {
    let conn = db.0.lock().unwrap_or_else(|p| p.into_inner());
    q_delete(&conn, id).map(|_| ()).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crud_roundtrip() {
        let mut path = std::env::temp_dir();
        path.push("pecule_assets_test.sqlite");
        let _ = std::fs::remove_file(&path);
        let db = Db::open(&path).expect("ouverture base");
        let conn = db.0.lock().unwrap();

        assert!(q_list(&conn).unwrap().is_empty());

        let id = q_insert(
            &conn,
            &NewAsset {
                nom: "Livret A".into(),
                kind: "epargne".into(),
                valeur: 12000.0,
                est_dette: false,
            },
        )
        .unwrap();
        assert!(id > 0);

        let debt = q_insert(
            &conn,
            &NewAsset {
                nom: "Prêt auto".into(),
                kind: "dette".into(),
                valeur: 5000.0,
                est_dette: true,
            },
        )
        .unwrap();
        assert!(debt > id);

        let list = q_list(&conn).unwrap();
        assert_eq!(list.len(), 2);
        assert!(!list[0].est_dette);
        assert!(list[1].est_dette);

        let mut a = list[0].clone();
        a.valeur = 13000.0;
        assert_eq!(q_update(&conn, &a).unwrap(), 1);
        assert!((q_list(&conn).unwrap()[0].valeur - 13000.0).abs() < 1e-9);

        assert_eq!(q_delete(&conn, debt).unwrap(), 1);
        assert_eq!(q_list(&conn).unwrap().len(), 1);

        drop(conn);
        let _ = std::fs::remove_file(&path);
    }
}
