use crate::messaging::BackendMessage;
use crate::views::register::register;
use cursive::{
    align::HAlign,
    event,
    views::{Dialog, SelectView},
    Cursive,
};
use tokio::sync::mpsc::UnboundedSender;

pub fn main_menu(siv: &mut Cursive, tx_backend: UnboundedSender<BackendMessage>) {
    // siv.set_autohide_menu(false);

    // let tx_backend_quit = tx_backend.clone();
    // siv.menubar().add_leaf("Quit", move |_| {
    //     tx_backend_quit.send(BackendMessage::Quit).unwrap();
    // });

    siv.add_global_callback(event::Event::CtrlChar('c'), |c| c.quit());

    // siv.add_global_callback(event::Key::Esc, |s| s.select_menubar());

    siv.add_layer(
        Dialog::new().title("SpaceTraders").content(
            SelectView::new()
                .h_align(HAlign::Center)
                .item_str("Register")
                .item_str("Login with Token")
                .item_str("Quit")
                .on_submit(move |siv, s: &str| match s {
                    "Register" => register(siv, tx_backend.clone()),
                    "Login with Token" => {}
                    "Quit" => {
                        tx_backend.send(BackendMessage::Quit).unwrap();
                    }
                    _ => unreachable!(),
                }),
        ),
    )
}
