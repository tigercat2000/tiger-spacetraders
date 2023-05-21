use std::cell::RefCell;

use cursive::reexports::ahash::HashMap;
use serde::{Deserialize, Serialize};
use spacetraders_sdk::{
    apis::{agents_api::get_my_agent, configuration::Configuration, default_api::register},
    models::{register_request::Faction, Agent, RegisterRequest},
};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::messaging::{BackendMessage, FrontendMessage};

thread_local! {
    pub static CONFIG: RefCell<Configuration> = RefCell::new(Configuration::new());
}

struct APISingleton;

impl APISingleton {
    fn set_defaults() {
        CONFIG.with(|f| {
            let mut config = f.borrow_mut();
            config.user_agent = Some("tigercat2000-client/1.0.0/rust".to_owned());
        })
    }

    fn get_config() -> Configuration {
        CONFIG.with(|f| f.borrow().clone())
    }

    fn set_token(token: String) {
        CONFIG.with(|f| f.borrow_mut().bearer_access_token = Some(token));
    }
}

pub fn backend(
    mut rx_backend: UnboundedReceiver<BackendMessage>,
    tx_frontend: UnboundedSender<FrontendMessage>,
) {
    APISingleton::set_defaults();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        loop {
            if let Some(x) = rx_backend.recv().await {
                match x {
                    BackendMessage::Register(callsign, faction) => {
                        try_register(&tx_frontend, callsign, faction).await;
                    }
                    BackendMessage::Quit => {
                        tx_frontend.send(FrontendMessage::Quit).unwrap();
                        break;
                    }
                    BackendMessage::TokenLogin(token) => {
                        try_login(&tx_frontend, token).await;
                    }
                    BackendMessage::RequestStatus => {}
                }
                // TODO:
            }
        }
    });
}

async fn try_register(
    tx_frontend: &UnboundedSender<FrontendMessage>,
    symbol: String,
    faction: Faction,
) {
    let config = APISingleton::get_config();

    let request = RegisterRequest { faction, symbol };

    match register(&config, Some(request)).await {
        Ok(res) => {
            tx_frontend
                .send(FrontendMessage::RegistrationDone(res.data))
                .unwrap();
        }
        Err(err) => {
            tx_frontend
                .send(FrontendMessage::RegistrationFailed(err.to_string()))
                .unwrap();
        }
    }
}

async fn try_login(tx_frontend: &UnboundedSender<FrontendMessage>, token: String) {
    APISingleton::set_token(token.clone());

    let config = APISingleton::get_config();

    match get_my_agent(&config).await {
        Ok(res) => {
            handle_recents(token, res.data.clone()).await;

            tx_frontend
                .send(FrontendMessage::LoggedIn(res.data))
                .unwrap();
        }
        Err(err) => {
            tx_frontend
                .send(FrontendMessage::LoginFailed(err.to_string()))
                .unwrap();
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RecentLogins {
    pub logins: HashMap<String, String>,
}

async fn handle_recents(token: String, agent: Box<Agent>) {
    let mut logins = match tokio::fs::read_to_string("recent.toml").await {
        Ok(s) => toml::from_str(&s).unwrap(),
        Err(_) => RecentLogins::default(),
    };

    logins.logins.insert(agent.symbol, token);

    tokio::fs::write("recent.toml", toml::to_string_pretty(&logins).unwrap())
        .await
        .unwrap();
}
