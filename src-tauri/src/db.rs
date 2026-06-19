//! Accès à la base **SQLite locale** de Pécule (via `rusqlite`, SQLite *bundled*).
//!
//! La base vit dans le dossier de données de l'application (résolu par Tauri).
//! Le schéma couvre dès maintenant les tables des trois piliers (§29.2) ; seules
//! les `settings` sont utilisées en v0.1. Le chiffrement optionnel (SQLCipher /
//! mot de passe maître, §16) viendra en v2 — voir `crypto.rs`.

use rusqlite::Connection;
use std::path::Path;
use std::sync::Mutex;

/// Connexion SQLite partagée, protégée par un mutex et gérée par l'état Tauri.
pub struct Db(pub Mutex<Connection>);

/// Schéma de la base (idempotent). Noms de colonnes en français, cf. §29.2.
const SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS settings (
    key   TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS accounts (
    id   INTEGER PRIMARY KEY,
    nom  TEXT NOT NULL,
    type TEXT NOT NULL                 -- PEA, CTO, AV, wallet…
);

CREATE TABLE IF NOT EXISTS holdings (
    id         INTEGER PRIMARY KEY,
    account_id INTEGER NOT NULL REFERENCES accounts(id),
    symbole    TEXT NOT NULL,
    type       TEXT NOT NULL,          -- action, etf, crypto…
    quantite   REAL NOT NULL,
    pru        REAL NOT NULL,          -- prix de revient unitaire
    devise     TEXT NOT NULL DEFAULT 'EUR'
);

CREATE TABLE IF NOT EXISTS transactions (
    id         INTEGER PRIMARY KEY,
    account_id INTEGER NOT NULL REFERENCES accounts(id),
    symbole    TEXT NOT NULL,
    sens       TEXT NOT NULL,          -- 'achat' | 'vente'
    quantite   REAL NOT NULL,
    prix       REAL NOT NULL,
    frais      REAL NOT NULL DEFAULT 0,
    date       TEXT NOT NULL           -- ISO 8601
);

CREATE TABLE IF NOT EXISTS dividends (
    id         INTEGER PRIMARY KEY,
    holding_id INTEGER NOT NULL REFERENCES holdings(id),
    montant    REAL NOT NULL,
    date       TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS subscriptions (
    id                   INTEGER PRIMARY KEY,
    nom                  TEXT NOT NULL,
    categorie            TEXT,
    montant              REAL NOT NULL,
    frequence            TEXT NOT NULL, -- 'mensuel' | 'annuel' | …
    prochain_prelevement TEXT,          -- ISO date
    actif                INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS simulations (
    id              INTEGER PRIMARY KEY,
    type            TEXT NOT NULL,
    parametres_json TEXT NOT NULL,
    cree_le         TEXT NOT NULL
);
"#;

impl Db {
    /// Ouvre (ou crée) la base au chemin donné et applique le schéma.
    pub fn open(path: &Path) -> rusqlite::Result<Self> {
        let conn = Connection::open(path)?;
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "foreign_keys", "ON")?;
        conn.execute_batch(SCHEMA)?;
        Ok(Db(Mutex::new(conn)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Valide le chemin de démarrage : ouverture + application du schéma + un
    /// aller-retour de réglage. Si le SQL était invalide, `open` échouerait —
    /// c'est exactement le risque de panique au lancement de l'application.
    #[test]
    fn open_creates_schema_and_settings_roundtrip() {
        let mut path = std::env::temp_dir();
        path.push("pecule_db_test_open.sqlite");
        let _ = std::fs::remove_file(&path);

        let db = Db::open(&path).expect("ouverture de la base");
        {
            let conn = db.0.lock().expect("verrou");
            for table in [
                "settings",
                "accounts",
                "holdings",
                "transactions",
                "dividends",
                "subscriptions",
                "simulations",
            ] {
                let n: i64 = conn
                    .query_row(
                        "SELECT count(*) FROM sqlite_master WHERE type='table' AND name=?1",
                        [table],
                        |r| r.get(0),
                    )
                    .expect("requête sqlite_master");
                assert_eq!(n, 1, "table manquante: {table}");
            }

            conn.execute("INSERT INTO settings (key, value) VALUES ('theme', 'dark')", [])
                .expect("insert setting");
            let v: String = conn
                .query_row("SELECT value FROM settings WHERE key = 'theme'", [], |r| {
                    r.get(0)
                })
                .expect("select setting");
            assert_eq!(v, "dark");
        }

        let _ = std::fs::remove_file(&path);
    }
}
