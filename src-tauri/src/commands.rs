use tauri::State;

#[cfg(not(mobile))]
use tauri::AppHandle;
use crate::state::AppState;

#[cfg(not(mobile))]
mod desktop_commands {
    use super::*;
    use crate::tray::sync_counter_indicators;

    #[tauri::command]
    pub fn increment_tray_counter(state: State<'_, AppState>, app: AppHandle) -> Result<u32, String> {
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
    pub fn decrement_tray_counter(state: State<'_, AppState>, app: AppHandle) -> Result<u32, String> {
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
}

#[cfg(mobile)]
mod mobile_commands {
    use super::*;

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
}

#[cfg(not(mobile))]
pub use desktop_commands::*;

#[cfg(mobile)]
pub use mobile_commands::*;
