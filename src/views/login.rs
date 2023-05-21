use cursive::{
    view::{Nameable, Resizable},
    views::{Dialog, EditView, ListView, SelectView},
    Cursive,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{backend_thread::RecentLogins, messaging::BackendMessage};

pub fn login(siv: &mut Cursive, tx_backend: UnboundedSender<BackendMessage>) {
    let logins = match std::fs::read_to_string("recent.toml") {
        Ok(s) => toml::from_str(&s).unwrap(),
        Err(_) => RecentLogins::default(),
    };

    if !logins.logins.is_empty() {
        let mut list = SelectView::new();

        for key in logins.logins.keys() {
            list.add_item_str(key);
        }

        list.add_item_str("-- Login With Token");

        list.set_on_submit(move |siv, sel: &String| match sel.as_str() {
            "-- Login With Token" => {
                siv.pop_layer();
                show_token_input(siv, tx_backend.clone());
            }
            other => {
                let token = logins.logins.get(other).unwrap();
                try_login(siv, tx_backend.clone(), token.clone());
            }
        });

        siv.add_layer(Dialog::new().title("SpaceTraders Login").content(list))
    } else {
        show_token_input(siv, tx_backend);
    }
}

fn show_token_input(siv: &mut Cursive, tx_backend: UnboundedSender<BackendMessage>) {
    siv.add_layer(
        Dialog::new()
            .title("SpaceTraders Login")
            .content(
                ListView::new().child(
                    "Token",
                    EditView::new()
                        .with_name("token")
                        .min_width(60)
                        .max_width(60),
                ),
            )
            .button("Back", |siv| {
                siv.pop_layer();
            })
            .button("Login", move |siv| {
                let token = siv
                    .call_on_name("token", |view: &mut EditView| view.get_content())
                    .unwrap();
                try_login(siv, tx_backend.clone(), (*token).clone());
            }),
    )
}

pub fn try_login(siv: &mut Cursive, tx_backend: UnboundedSender<BackendMessage>, token: String) {
    tx_backend.send(BackendMessage::TokenLogin(token)).unwrap();
    siv.add_layer(Dialog::info("Logging in..."))
}
