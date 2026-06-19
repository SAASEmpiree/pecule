//! Commandes de réglages persistés dans la table `settings` (clé/valeur).
//! Destinées aux préférences durables (devise de base, cours on/off…). Le thème
//! et la langue sont gérés côté front pour une bascule instantanée.

use crate::db::Db;
use rusqlite::OptionalExtension;
use tauri::State;

#[tauri::command]
pub fn get_setting(db: State<Db>, key: String) -> Result<Option<String>, String> {
    // Récupère la connexion même si un thread a paniqué en la détenant
    // (mutex empoisonné) : les données SQLite restent valides.
    let conn = db.0.lock().unwrap_or_else(|poison| poison.into_inner());
    conn.query_row("SELECT value FROM settings WHERE key = ?1", [key], |row| {
        row.get::<_, String>(0)
    })
    .optional()
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_setting(db: State<Db>, key: String, value: String) -> Result<(), String> {
    // Récupère la connexion même si un thread a paniqué en la détenant
    // (mutex empoisonné) : les données SQLite restent valides.
    let conn = db.0.lock().unwrap_or_else(|poison| poison.into_inner());
    conn.execute(
        "INSERT INTO settings (key, value) VALUES (?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        (key, value),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
