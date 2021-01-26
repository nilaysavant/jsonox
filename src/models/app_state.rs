#[derive(Debug)]
pub struct AppState {
    mode: AppMode,
}

#[derive(Debug)]
pub enum AppMode {
    Normal,
    ReadOnly,
}
