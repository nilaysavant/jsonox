/// State of the Application
#[derive(Debug)]
pub struct AppState {
    mode: AppMode,
}

/// AppMode: Normal or ReadOnly
#[derive(Debug)]
pub enum AppMode {
    Normal,
    ReadOnly,
}
