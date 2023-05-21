use spacetraders_sdk::models::{register_request::Faction, Agent, Register201ResponseData};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum BackendMessage {
    Register(String, Faction),
    TokenLogin(String),
    RequestStatus,
    Quit,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum FrontendMessage {
    LoggedIn(Box<Agent>),
    LoginFailed(String),
    Refresh,
    Quit,
    /// Contains the token
    RegistrationDone(Box<Register201ResponseData>),
    /// Contains the error message
    RegistrationFailed(String),
}
