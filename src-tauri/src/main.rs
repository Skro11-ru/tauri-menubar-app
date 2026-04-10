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

#[cfg(not(mobile))]
use state::AppState;

#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;
#[cfg(not(mobile))]
use tauri_plugin_global_shortcut::ShortcutState;

#[cfg(not(mobile))]
fn main() {
    let global_shortcut = tauri_plugin_global_shortcut::Builder::new()
        .with_shortcut("CommandOrControl+Shift+Space")
        .expect("invalid global shortcut")
        .with_handler(|app, _shortcut, event| {
            if event.state == ShortcutState::Pressed {
                window::toggle_main_window(app);
            }
        })
        .build();

    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            commands::increment_tray_counter,
            commands::decrement_tray_counter,
            commands::reset_tray_counter,
            commands::set_badge_count,
            commands::pin_window,
            commands::unpin_window,
            commands::cmd_front_hide,
        ])
        .on_window_event(window::handle_window_event)
        .setup(move |app| {
            #[cfg(target_os = "macos")]
            {
                app.set_activation_policy(ActivationPolicy::Accessory);
            }

            if let Err(err) = app.handle().plugin(global_shortcut) {
                eprintln!("failed to register global shortcut plugin: {err}");
            }

            if let Err(err) = tray::setup_tray(app.handle()) {
                eprintln!("failed to setup tray: {err}");
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
