use cursive::theme::{BaseColor, BorderStyle, Color, PaletteColor};
use eyre::Result;

mod async_message;
mod sdk_layer;
mod views;

fn main() -> Result<()> {
    let mut siv = cursive::default();

    let (async_tx, async_rx) = tokio::sync::mpsc::unbounded_channel();

    let tokio_siv_cb = siv.cb_sink().clone();
    std::thread::spawn(move || sdk_layer::sdk_layer(tokio_siv_cb, async_rx));

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
