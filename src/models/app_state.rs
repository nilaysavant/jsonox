/// State of the Application
#[derive(Debug, Clone)]
pub struct AppState {
    pub mode: AppMode,
}

/// AppMode: Normal or ReadOnly
#[derive(Debug, Clone)]
pub enum AppMode {
    Normal,
    ReadOnly,
}
