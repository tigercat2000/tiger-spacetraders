// use cursive::{
//     theme::Style,
//     utils::span::SpannedString,
//     view::{Nameable, Resizable},
//     views::{Dialog, EditView, ListView, ProgressBar, SelectView},
//     Cursive,
// };
// use tokio::sync::mpsc::UnboundedSender;

// use crate::async_message::{AsyncMessage, AsyncMessageType};

// pub fn show_login(siv: &mut Cursive, async_tx: UnboundedSender<AsyncMessage>) {
//     siv.add_layer(
//         Dialog::new().title("SpaceTraders").content(
//             SelectView::new()
//                 .item_str("Register")
//                 .item_str("Login with Token")
//                 .item_str("Quit")
//                 .on_submit(move |siv, selection: &str| match selection {
//                     "Register" => {
//                         registration(siv, async_tx.clone());
//                     }
//                     "Login with Token" => {
//                         token_login(siv);
//                     }
//                     "Quit" => siv.quit(),
//                     _ => unreachable!("No such item"),
//                 }),
//         ),
//     );
// }

// fn registration(siv: &mut Cursive, async_tx: UnboundedSender<AsyncMessage>) {
//     let async_tx_one = async_tx.clone();
//     let async_tx_two = async_tx;

//     siv.add_layer(
//         Dialog::new()
//             .title("SpaceTraders Registration")
//             .content(
//                 ListView::new().child(
//                     "Call Sign",
//                     EditView::new()
//                         .on_submit(move |siv, cs| register(siv, cs, async_tx_one.clone()))
//                         .on_edit_mut(|siv, content, _| {
//                             siv.call_on_name("csign", |csign: &mut EditView| {
//                                 csign.set_content(content.to_ascii_uppercase())
//                             });
//                         })
//                         .max_content_width(14)
//                         .with_name("csign")
//                         .min_width(15)
//                         .max_width(15),
//                 ),
//             )
//             .button("Back", |siv| {
//                 siv.pop_layer();
//             })
//             .button("Register", move |siv| {
//                 let name = siv
//                     .call_on_name("csign", |view: &mut EditView| view.get_content())
//                     .unwrap();

//                 register(siv, name.as_str(), async_tx_two.clone());
//             }),
//     );
// }

// fn register(siv: &mut Cursive, callsign: &str, async_tx: UnboundedSender<AsyncMessage>) {
//     let len = callsign.chars().count();

//     if !(3..=14).contains(&len) {
//         siv.add_layer(Dialog::info(SpannedString::styled(
//             "Error: Callsign must be 3-14 characters",
//             Style::highlight(),
//         )));
//         return;
//     }

//     async_tx
//         .send(AsyncMessage {
//             typ: AsyncMessageType::Register(callsign.into()),
//         })
//         .unwrap();
// }

// fn token_login(siv: &mut Cursive) {
//     siv.add_layer(
//         Dialog::new()
//             .title("SpaceTraders Token Login")
//             .button("Back", |siv| {
//                 siv.pop_layer();
//             }),
//     );
// }
