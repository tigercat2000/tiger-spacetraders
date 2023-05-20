use std::cell::RefCell;

use spacetraders_sdk::{
    apis::{configuration::Configuration, default_api::register},
    models::{register_request::Faction, RegisterRequest},
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
                        try_register(&tx_frontend, callsign, faction).await
                    }
                    BackendMessage::Quit => {
                        tx_frontend.send(FrontendMessage::Quit).unwrap();
                        break;
                    }
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
