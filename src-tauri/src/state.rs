use std::sync::{Mutex, OnceLock};

#[derive(Default)]
pub struct AppState {
    pub counter: Mutex<u32>,
}

pub static AUTO_HIDE_ENABLED: OnceLock<Mutex<bool>> = OnceLock::new();

pub fn auto_hide_enabled() -> bool {
    if let Ok(value) = AUTO_HIDE_ENABLED.get_or_init(|| Mutex::new(true)).lock() {
        *value
    } else {
        true
    }
}

pub fn set_auto_hide(enabled: bool) {
    if let Ok(mut value) = AUTO_HIDE_ENABLED.get_or_init(|| Mutex::new(true)).lock() {
        *value = enabled;
    }
}
