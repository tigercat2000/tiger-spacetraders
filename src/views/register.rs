use cursive::{
    theme::Style,
    utils::span::SpannedString,
    view::{Nameable, Resizable},
    views::{Dialog, EditView, ListView, SelectView, TextView},
    Cursive,
};
use spacetraders_sdk::models::{register_request::Faction, Register201ResponseData};
use tokio::sync::mpsc::UnboundedSender;

use crate::messaging::BackendMessage;

pub fn register(siv: &mut Cursive, tx_backend: UnboundedSender<BackendMessage>) {
    siv.add_layer(
        Dialog::new()
            .title("SpaceTraders Registration")
            .content(
                ListView::new()
                    .child(
                        "Call Sign",
                        EditView::new()
                            .on_submit(|siv, _| {
                                siv.focus_name("faction").unwrap();
                            })
                            .on_edit_mut(|siv, content, _| {
                                siv.call_on_name("csign", |csign: &mut EditView| {
                                    csign.set_content(content.to_ascii_uppercase())
                                });
                            })
                            .max_content_width(14)
                            .with_name("csign")
                            .min_width(15)
                            .max_width(15),
                    )
                    .child(
                        "Faction",
                        SelectView::new()
                            .item_str("COSMIC")
                            .item_str("VOID")
                            .item_str("GALACTIC")
                            .item_str("QUANTUM")
                            .item_str("DOMINION")
                            .popup()
                            .with_name("faction"),
                    ),
            )
            .button("Back", |siv| {
                siv.pop_layer();
            })
            .button("Register", move |siv| {
                try_register(siv, tx_backend.clone());
            }),
    );
}

pub fn register_error(siv: &mut Cursive, error: String) {
    siv.pop_layer();
    siv.add_layer(Dialog::info(error).title("Registration Error"));
}

pub fn register_success(siv: &mut Cursive, data: Box<Register201ResponseData>) {
    std::fs::write("./debug.log", format!("{:#?}", data));

    // siv.pop_layer();
    // siv.add_layer(Dialog::info("Your token"));
}

fn try_register(siv: &mut Cursive, tx_backend: UnboundedSender<BackendMessage>) {
    let callsign = siv
        .call_on_name("csign", |view: &mut EditView| view.get_content())
        .unwrap();

    let len = callsign.chars().count();

    if !(3..=14).contains(&len) {
        siv.add_layer(Dialog::info(SpannedString::styled(
            "Error: Callsign must be 3-14 characters",
            Style::highlight(),
        )));
        return;
    }

    let faction = siv
        .call_on_name("faction", |view: &mut SelectView| view.selection())
        .unwrap()
        .unwrap();

    let faction = match faction.as_str() {
        "COSMIC" => Faction::Cosmic,
        "VOID" => Faction::Void,
        "GALACTIC" => Faction::Galactic,
        "QUANTUM" => Faction::Quantum,
        "DOMINION" => Faction::Dominion,
        _ => unreachable!(),
    };

    tx_backend
        .send(BackendMessage::Register((*callsign).clone(), faction))
        .unwrap();

    siv.add_layer(Dialog::new().content(TextView::new("Contacting server...")));
}
