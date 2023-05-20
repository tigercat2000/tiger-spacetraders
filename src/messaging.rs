use spacetraders_sdk::models::{register_request::Faction, Register201ResponseData};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum BackendMessage {
    Register(String, Faction),
    Quit,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum FrontendMessage {
    Refresh,
    Quit,
    /// Contains the token
    RegistrationDone(Box<Register201ResponseData>),
    /// Contains the error message
    RegistrationFailed(String),
}
