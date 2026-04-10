use tauri::State;

use crate::state::AppState;
#[cfg(not(mobile))]
use tauri::AppHandle;

#[cfg(not(mobile))]
mod desktop_commands {
    use super::*;
    use crate::tray::sync_counter_indicators;
    use crate::window;
    use tauri::Manager;

    #[tauri::command]
    pub fn increment_tray_counter(
        state: State<'_, AppState>,
        app: AppHandle,
    ) -> Result<u32, String> {
        let mut counter = state
            .counter
            .lock()
            .map_err(|e| format!("Failed to lock counter: {e}"))?;
        *counter = counter.saturating_add(1);
        let count = *counter;
        sync_counter_indicators(&app, count)?;
        Ok(count)
    }

    #[tauri::command]
    pub fn decrement_tray_counter(
        state: State<'_, AppState>,
        app: AppHandle,
    ) -> Result<u32, String> {
        let mut counter = state
            .counter
            .lock()
            .map_err(|e| format!("Failed to lock counter: {e}"))?;
        *counter = counter.saturating_sub(1);
        let count = *counter;
        sync_counter_indicators(&app, count)?;
        Ok(count)
    }

    #[tauri::command]
    pub fn reset_tray_counter(state: State<'_, AppState>, app: AppHandle) -> Result<u32, String> {
        let mut counter = state
            .counter
            .lock()
            .map_err(|e| format!("Failed to lock counter: {e}"))?;
        *counter = 0;
        let count = *counter;
        sync_counter_indicators(&app, count)?;
        Ok(count)
    }

    #[tauri::command]
    pub fn set_badge_count(app: AppHandle, count: u32) -> Result<(), String> {
        sync_counter_indicators(&app, count)?;
        Ok(())
    }

    #[tauri::command]
    pub fn cmd_front_hide(app: AppHandle) -> Result<(), String> {
        let window = app
            .get_webview_window("main")
            .ok_or("Main window not found")?;

        window
            .hide()
            .map_err(|e| format!("Failed to hide window: {e}"))?;

        Ok(())
    }

    #[tauri::command]
    pub fn pin_window(app: AppHandle) -> Result<(), String> {
        if let Some(window) = app.get_webview_window("main") {
            window
                .set_resizable(true)
                .map_err(|e| format!("Failed to set resizable: {e}"))?;
            window
                .set_always_on_top(false)
                .map_err(|e| format!("Failed to set always on top: {e}"))?;
            window
                .set_skip_taskbar(false)
                .map_err(|e| format!("Failed to set skip taskbar: {e}"))?;
        }
        Ok(())
    }

    #[tauri::command]
    pub fn unpin_window(app: AppHandle) -> Result<(), String> {
        if let Some(window) = app.get_webview_window("main") {
            window
                .set_resizable(false)
                .map_err(|e| format!("Failed to set resizable: {e}"))?;
            window
                .set_always_on_top(true)
                .map_err(|e| format!("Failed to set always on top: {e}"))?;
            window
                .set_skip_taskbar(true)
                .map_err(|e| format!("Failed to set skip taskbar: {e}"))?;
        }
        Ok(())
    }
}

#[cfg(mobile)]
mod mobile_commands {
    use super::*;
    use tauri::AppHandle;

    #[tauri::command]
    pub fn increment_tray_counter(state: State<'_, AppState>) -> Result<u32, String> {
        let mut counter = state
            .counter
            .lock()
            .map_err(|e| format!("Failed to lock counter: {e}"))?;
        *counter = counter.saturating_add(1);
        let count = *counter;
        Ok(count)
    }

    #[tauri::command]
    pub fn decrement_tray_counter(state: State<'_, AppState>) -> Result<u32, String> {
        let mut counter = state
            .counter
            .lock()
            .map_err(|e| format!("Failed to lock counter: {e}"))?;
        *counter = counter.saturating_sub(1);
        let count = *counter;
        Ok(count)
    }

    #[tauri::command]
    pub fn reset_tray_counter(state: State<'_, AppState>) -> Result<u32, String> {
        let mut counter = state
            .counter
            .lock()
            .map_err(|e| format!("Failed to lock counter: {e}"))?;
        *counter = 0;
        let count = *counter;
        Ok(count)
    }

    /// Badge count is a desktop-only feature on Android
    #[tauri::command]
    pub fn set_badge_count(_app: AppHandle, _count: u32) -> Result<(), String> {
        Ok(())
    }

    /// Window pinning is a desktop-only feature on Android
    #[tauri::command]
    pub fn pin_window(_app: AppHandle) -> Result<(), String> {
        Ok(())
    }

    /// Window unpinning is a desktop-only feature on Android
    #[tauri::command]
    pub fn unpin_window(_app: AppHandle) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(not(mobile))]
pub use desktop_commands::*;

#[cfg(mobile)]
pub use mobile_commands::*;
