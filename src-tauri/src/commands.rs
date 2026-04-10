use tauri::{AppHandle, State};
use crate::state::AppState;
use crate::tray::sync_counter_indicators;

#[tauri::command]
pub fn increment_tray_counter(state: State<'_, AppState>, app: AppHandle) -> Result<u32, String> {
    let mut counter = state
        .counter
        .lock()
        .map_err(|e| format!("Failed to lock counter: {e}"))?;
    *counter = counter.saturating_add(1);
    let count = *counter;
    drop(counter);
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
    drop(counter);
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
    drop(counter);
    sync_counter_indicators(&app, count)?;
    Ok(count)
}
