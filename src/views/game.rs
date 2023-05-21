use cursive::{
    view::Scrollable,
    views::{Dialog, LinearLayout, ListView, TextView},
    Cursive,
};
use spacetraders_sdk::models::Agent;
use tokio::sync::mpsc::UnboundedSender;

use crate::messaging::BackendMessage;

pub fn main_game_screen(
    siv: &mut Cursive,
    _tx_backend: UnboundedSender<BackendMessage>,
    agent: Agent,
) {
    siv.add_active_screen();

    siv.add_fullscreen_layer(
        LinearLayout::vertical()
            .child(
                Dialog::new().title("Agent Info").content(
                    ListView::new()
                        .child("Welcome to SpaceTraders", TextView::new(agent.symbol))
                        .child("Credits", TextView::new(agent.credits.to_string()))
                        .child("You are stationed at", TextView::new(agent.headquarters)),
                ),
            )
            .scrollable(),
    )
}
