//! Commandes du **budget** (v0.5) : CRUD des dépenses et revenus, persistés en
//! SQLite local. Les totaux mensuels et la répartition par catégorie sont
//! calculés côté front.

use crate::db::Db;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use tauri::State;

/// Une opération de budget (dépense ou revenu).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Expense {
    pub id: i64,
    /// Date ISO 8601 (AAAA-MM-JJ).
    pub date: String,
    pub montant: f64,
    pub categorie: String,
    /// "depense" | "revenu".
    pub kind: String,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewExpense {
    pub date: String,
    pub montant: f64,
    pub categorie: String,
    pub kind: String,
    pub note: Option<String>,
}

fn q_list(conn: &Connection) -> rusqlite::Result<Vec<Expense>> {
    let mut stmt = conn.prepare(
        "SELECT id, date, montant, categorie, kind, note
         FROM expenses ORDER BY date DESC, id DESC",
    )?;
    let rows = stmt.query_map([], |r| {
        Ok(Expense {
            id: r.get(0)?,
            date: r.get(1)?,
            montant: r.get(2)?,
            categorie: r.get(3)?,
            kind: r.get(4)?,
            note: r.get(5)?,
        })
    })?;
    rows.collect()
}

fn q_insert(conn: &Connection, e: &NewExpense) -> rusqlite::Result<i64> {
    conn.execute(
        "INSERT INTO expenses (date, montant, categorie, kind, note)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![e.date, e.montant, e.categorie, e.kind, e.note],
    )?;
    Ok(conn.last_insert_rowid())
}

fn q_update(conn: &Connection, e: &Expense) -> rusqlite::Result<usize> {
    conn.execute(
        "UPDATE expenses SET date=?2, montant=?3, categorie=?4, kind=?5, note=?6 WHERE id=?1",
        params![e.id, e.date, e.montant, e.categorie, e.kind, e.note],
    )
}

fn q_delete(conn: &Connection, id: i64) -> rusqlite::Result<usize> {
    conn.execute("DELETE FROM expenses WHERE id=?1", params![id])
}

#[tauri::command]
pub fn list_expenses(db: State<Db>) -> Result<Vec<Expense>, String> {
    let conn = db.0.lock().unwrap_or_else(|p| p.into_inner());
    q_list(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn add_expense(db: State<Db>, expense: NewExpense) -> Result<i64, String> {
    let conn = db.0.lock().unwrap_or_else(|p| p.into_inner());
    q_insert(&conn, &expense).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_expense(db: State<Db>, expense: Expense) -> Result<(), String> {
    let conn = db.0.lock().unwrap_or_else(|p| p.into_inner());
    q_update(&conn, &expense).map(|_| ()).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_expense(db: State<Db>, id: i64) -> Result<(), String> {
    let conn = db.0.lock().unwrap_or_else(|p| p.into_inner());
    q_delete(&conn, id).map(|_| ()).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crud_roundtrip() {
        let mut path = std::env::temp_dir();
        path.push("pecule_budget_test.sqlite");
        let _ = std::fs::remove_file(&path);
        let db = Db::open(&path).expect("ouverture base");
        let conn = db.0.lock().unwrap();

        assert!(q_list(&conn).unwrap().is_empty());

        let id = q_insert(
            &conn,
            &NewExpense {
                date: "2026-06-15".into(),
                montant: 42.5,
                categorie: "Courses".into(),
                kind: "depense".into(),
                note: Some("Supermarché".into()),
            },
        )
        .unwrap();
        assert!(id > 0);
        q_insert(
            &conn,
            &NewExpense {
                date: "2026-06-01".into(),
                montant: 2500.0,
                categorie: "Salaire".into(),
                kind: "revenu".into(),
                note: None,
            },
        )
        .unwrap();

        let list = q_list(&conn).unwrap();
        assert_eq!(list.len(), 2);
        // Tri par date décroissante : le 15 juin avant le 1er juin.
        assert_eq!(list[0].date, "2026-06-15");
        assert_eq!(list[0].kind, "depense");
        assert_eq!(list[0].note.as_deref(), Some("Supermarché"));
        assert_eq!(list[1].note, None);

        let mut e = list[0].clone();
        e.montant = 50.0;
        assert_eq!(q_update(&conn, &e).unwrap(), 1);
        assert!((q_list(&conn).unwrap()[0].montant - 50.0).abs() < 1e-9);

        assert_eq!(q_delete(&conn, id).unwrap(), 1);
        assert_eq!(q_list(&conn).unwrap().len(), 1);

        drop(conn);
        let _ = std::fs::remove_file(&path);
    }
}
