#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod state;
#[cfg(not(mobile))]
mod tray;
#[cfg(not(mobile))]
mod window;

use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            commands::increment_tray_counter,
            commands::decrement_tray_counter,
            commands::reset_tray_counter
        ])
        .setup(|_app| {
            // Initialize mobile-specific plugins here
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
