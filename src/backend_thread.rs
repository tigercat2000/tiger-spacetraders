use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::messaging::{BackendMessage, FrontendMessage};

pub fn backend(
    mut rx_backend: UnboundedReceiver<BackendMessage>,
    _tx_frontend: UnboundedSender<FrontendMessage>,
) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        loop {
            if let Some(x) = rx_backend.recv().await {
                match x {
                    BackendMessage::Register(_) => todo!(),
                    BackendMessage::Quit => break,
                }
                // TODO:
            }
        }
    });
}
