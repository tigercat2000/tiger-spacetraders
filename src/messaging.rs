#[derive(Debug, Clone)]
pub enum BackendMessage {
    Register(String),
    Quit,
}

#[derive(Debug, Clone)]
pub enum FrontendMessage {
    Noop,
    Refresh,
    /// Contains the token
    RegistrationDone(String),
}
