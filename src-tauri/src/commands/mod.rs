//! Commandes Tauri (`#[tauri::command]`) — la frontière IPC entre le frontend
//! Svelte et le backend Rust. Chaque sous-module regroupe un domaine.

pub mod networth;
pub mod portfolio;
pub mod settings;
pub mod simulate;
pub mod subscriptions;
