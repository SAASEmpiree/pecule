//! Point d'entrée de la bibliothèque applicative de Pécule (backend Rust).
//!
//! Responsabilités : ouvrir la base locale, enregistrer les commandes IPC et
//! lancer la fenêtre Tauri. La logique financière vit dans `finance` (pure et
//! testée) ; l'IPC dans `commands`.

mod alerts;
mod commands;
mod crypto;
mod db;
mod finance;
mod quotes;

use db::Db;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Base locale dans le dossier de données de l'application.
            let dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&dir)?;
            let db = Db::open(&dir.join("pecule.db"))?;
            app.manage(db);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::simulate::simulate_compound,
            commands::simulate::simulate_savings,
            commands::simulate::simulate_loan,
            commands::settings::get_setting,
            commands::settings::set_setting,
            commands::portfolio::list_holdings,
            commands::portfolio::add_holding,
            commands::portfolio::update_holding,
            commands::portfolio::delete_holding,
            commands::subscriptions::list_subscriptions,
            commands::subscriptions::add_subscription,
            commands::subscriptions::update_subscription,
            commands::subscriptions::delete_subscription,
            commands::networth::list_assets,
            commands::networth::add_asset,
            commands::networth::update_asset,
            commands::networth::delete_asset,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
