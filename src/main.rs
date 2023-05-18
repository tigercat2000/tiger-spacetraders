use cursive::{
    theme::{BaseColor, BorderStyle, Color, PaletteColor},
    views::Dialog,
    CbSink,
};
use eyre::Result;
use tokio::sync::mpsc::UnboundedReceiver;

use crate::async_message::AsyncMessage;

mod async_message;
mod views;

fn main() -> Result<()> {
    let mut siv = cursive::default();

    let (async_tx, async_rx) = tokio::sync::mpsc::unbounded_channel();

    let tokio_siv_cb = siv.cb_sink().clone();
    std::thread::spawn(move || tokio_main(tokio_siv_cb, async_rx));

    let mut theme = siv.current_theme().clone();
    theme.shadow = false;
    theme.borders = BorderStyle::Simple;

    theme.palette[PaletteColor::Background] = Color::Dark(BaseColor::Black);
    theme.palette[PaletteColor::View] = Color::Dark(BaseColor::Black);
    theme.palette[PaletteColor::Primary] = Color::Dark(BaseColor::White);
    theme.palette[PaletteColor::Secondary] = Color::Dark(BaseColor::Blue);
    theme.palette[PaletteColor::Tertiary] = Color::Dark(BaseColor::Blue);
    theme.palette[PaletteColor::Highlight] = Color::Dark(BaseColor::Red);

    siv.set_theme(theme);

    views::login::show_login(&mut siv, async_tx);

    siv.run();

    Ok(())
}

fn tokio_main(siv_cb: CbSink, mut async_rx: UnboundedReceiver<AsyncMessage>) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        loop {
            if let Some(AsyncMessage { typ }) = async_rx.recv().await {
                siv_cb
                    .send(Box::new(move |siv| {
                        siv.add_layer(Dialog::info(format!(
                            "Hello from tokio thread, got {:?}",
                            typ
                        )));
                    }))
                    .unwrap();
            }
        }
    })
}
