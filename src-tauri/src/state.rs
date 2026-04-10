use std::sync::Mutex;

#[derive(Default)]
pub struct AppState {
    pub counter: Mutex<u32>,
}
