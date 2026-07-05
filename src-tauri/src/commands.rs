use tauri::State;

use crate::state::AppState;
#[cfg(not(mobile))]
use tauri::AppHandle;

#[macro_export]
macro_rules! generate_app_invoke_handler {
    () => {
        tauri::generate_handler![
            crate::commands::increment_tray_counter,
            crate::commands::decrement_tray_counter,
            crate::commands::reset_tray_counter,
            crate::commands::set_badge_count,
            crate::commands::pin_window,
            crate::commands::unpin_window,
            crate::commands::cmd_front_hide,
        ]
    };
}

#[cfg(not(mobile))]
mod desktop_commands {
    use super::*;
    use crate::tray::sync_counter_indicators;
    use tauri::Manager;

    fn update_counter(
        state: State<'_, AppState>,
        app: AppHandle,
        update: impl FnOnce(u32) -> u32,
    ) -> Result<u32, String> {
        let mut counter = state
            .counter
            .lock()
            .map_err(|e| format!("Failed to lock counter: {e}"))?;

        *counter = update(*counter);
        let count = *counter;

        sync_counter_indicators(&app, count)?;

        Ok(count)
    }

    #[tauri::command]
    pub fn increment_tray_counter(
        state: State<'_, AppState>,
        app: AppHandle,
    ) -> Result<u32, String> {
        update_counter(state, app, |count| count.saturating_add(1))
    }

    #[tauri::command]
    pub fn decrement_tray_counter(
        state: State<'_, AppState>,
        app: AppHandle,
    ) -> Result<u32, String> {
        update_counter(state, app, |count| count.saturating_sub(1))
    }

    #[tauri::command]
    pub fn reset_tray_counter(state: State<'_, AppState>, app: AppHandle) -> Result<u32, String> {
        update_counter(state, app, |_| 0)
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

    fn update_counter(
        state: State<'_, AppState>,
        update: impl FnOnce(u32) -> u32,
    ) -> Result<u32, String> {
        let mut counter = state
            .counter
            .lock()
            .map_err(|e| format!("Failed to lock counter: {e}"))?;

        *counter = update(*counter);

        Ok(*counter)
    }

    #[tauri::command]
    pub fn increment_tray_counter(state: State<'_, AppState>) -> Result<u32, String> {
        update_counter(state, |count| count.saturating_add(1))
    }

    #[tauri::command]
    pub fn decrement_tray_counter(state: State<'_, AppState>) -> Result<u32, String> {
        update_counter(state, |count| count.saturating_sub(1))
    }

    #[tauri::command]
    pub fn reset_tray_counter(state: State<'_, AppState>) -> Result<u32, String> {
        update_counter(state, |_| 0)
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

    /// Hiding the desktop popover is not available on mobile.
    #[tauri::command]
    pub fn cmd_front_hide(_app: AppHandle) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(not(mobile))]
pub use desktop_commands::*;

#[cfg(mobile)]
pub use mobile_commands::*;
