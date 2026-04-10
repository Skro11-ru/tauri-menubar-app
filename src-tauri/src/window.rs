use tauri::{AppHandle, Emitter, LogicalPosition, Manager, WindowEvent};

pub fn show_main_window<R: tauri::Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or("Main window not found")?;

    if let Some(tray) = app.tray_by_id("main") {
        if let Ok(Some(rect)) = tray.rect() {
            let scale_factor = window.scale_factor().unwrap_or(1.0);
            let tray_position = rect.position.to_logical::<f64>(scale_factor);
            let tray_size = rect.size.to_logical::<f64>(scale_factor);
            let window_size = window
                .outer_size()
                .unwrap_or_default()
                .to_logical::<f64>(scale_factor);

            let x = tray_position.x + (tray_size.width / 2.0) - (window_size.width / 2.0);
            let mut y = tray_position.y - window_size.height;

            #[cfg(target_os = "windows")]
            if y < 0.0 {
                y = tray_position.y + tray_size.height;
            }

            #[cfg(target_os = "macos")]
            if y < 0.0 {
                y = tray_position.y;
            }

            window
                .set_position(LogicalPosition::new(x, y))
                .map_err(|e| format!("Failed to set window position: {e}"))?;
        }
    }

    window
        .show()
        .map_err(|e| format!("Failed to show window: {e}"))?;
    window
        .set_focus()
        .map_err(|e| format!("Failed to focus window: {e}"))?;
    window
        .emit("focus-search-input", ())
        .map_err(|e| format!("Failed to emit focus event: {e}"))?;

    Ok(())
}

pub fn toggle_main_window<R: tauri::Runtime>(app: &AppHandle<R>) {
    let Some(window) = app.get_webview_window("main") else {
        return;
    };

    if window.is_visible().unwrap_or(false) {
        let _ = window.hide();
        return;
    }

    let _ = show_main_window(app);
}

pub fn handle_window_event(window: &tauri::Window, event: &WindowEvent) {
    if window.label() != "main" {
        return;
    }

    if let WindowEvent::Focused(false) = event {
        if crate::state::auto_hide_enabled() && window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        }
    }
}
