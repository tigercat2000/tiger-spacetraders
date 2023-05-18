use crate::async_message::{AsyncMessage, AsyncMessageType};
use cursive::{view::Nameable, views::Dialog, CbSink};
use spacetraders_sdk::{
    apis::{configuration::Configuration, default_api::register},
    models::{register_request::Faction, RegisterRequest},
};
use tokio::sync::mpsc::UnboundedReceiver;

pub fn sdk_layer(siv_cb: CbSink, mut async_rx: UnboundedReceiver<AsyncMessage>) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let configuration = Configuration::default();

        loop {
            if let Some(AsyncMessage { typ }) = async_rx.recv().await {
                match typ {
                    AsyncMessageType::Register(name) => {
                        siv_cb
                            .send(Box::new(move |siv| {
                                siv.add_layer(
                                    Dialog::info("Registering...").with_name("AsyncRegBox"),
                                );
                            }))
                            .unwrap();

                        match register(
                            &configuration,
                            Some(RegisterRequest::new(Faction::Cosmic, name)),
                        )
                        .await
                        {
                            Ok(res) => {
                                siv_cb
                                    .send(Box::new(|siv| {
                                        siv.pop_layer();
                                        siv.add_layer(Dialog::info("You registered!"))
                                    }))
                                    .unwrap();
                            }
                            Err(_) => todo!(),
                        }
                    }
                }
            }
        }
    })
}
