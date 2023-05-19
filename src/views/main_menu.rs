use cursive::{views::Dialog, Cursive};
use tokio::sync::mpsc::UnboundedSender;

use crate::messaging::BackendMessage;

pub fn main_menu(siv: &mut Cursive, tx_backend: UnboundedSender<BackendMessage>) {
    siv.add_layer(
        Dialog::new()
            .title("SpaceTraders")
            .button("meow", move |_| {
                tx_backend.send(BackendMessage::Quit).unwrap();
            }),
    )
}
