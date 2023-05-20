use std::{
    ops::DerefMut,
    time::{Duration, Instant},
};

use cursive::{
    theme::{BaseColor, BorderStyle, Color, PaletteColor, Style},
    view::{Nameable, Resizable},
    views::{Dialog, LinearLayout, TextView},
    Cursive, CursiveRunnable, CursiveRunner,
};
use eyre::Result;
use tokio::sync::mpsc::{error::TryRecvError, UnboundedReceiver};

use crate::{
    backend_thread::backend,
    messaging::FrontendMessage,
    views::register::{register_error, register_success},
};

mod backend_thread;
mod messaging;
mod views;

fn main() -> Result<()> {
    let (tx_backend, rx_backend) = tokio::sync::mpsc::unbounded_channel();
    let (tx_frontend, rx_frontend) = tokio::sync::mpsc::unbounded_channel();

    std::thread::spawn(move || {
        backend(rx_backend, tx_frontend);
    });

    let mut siv = cursive::default();
    setup_theme(&mut siv);

    views::main_menu::main_menu(&mut siv, tx_backend);

    event_loop(siv, rx_frontend);

    Ok(())
}

fn setup_theme(siv: &mut Cursive) {
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
}

fn event_loop(mut siv: CursiveRunnable, mut rx_frontend: UnboundedReceiver<FrontendMessage>) {
    let mut runner = siv.runner();

    while runner.is_running() {
        runner.step();

        match rx_frontend.try_recv() {
            Ok(message) => {
                match message {
                    FrontendMessage::Refresh => runner.refresh(),
                    FrontendMessage::Quit => {
                        break;
                    }
                    FrontendMessage::RegistrationDone(data) => {
                        register_success(runner.deref_mut(), data);
                    }
                    FrontendMessage::RegistrationFailed(error) => {
                        register_error(runner.deref_mut(), error)
                    }
                };
                runner.refresh();
            }
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => {
                backend_crash(&mut runner);
            }
        }
    }
}

fn backend_crash(runner: &mut CursiveRunner<&mut Cursive>) {
    runner.add_active_screen();

    runner.add_layer(
        Dialog::new().title("FATAL ERROR").content(
            LinearLayout::horizontal()
                .child(
                    TextView::new("ERROR: Backend crashed, shutting down in")
                        .style(Style::highlight()),
                )
                .child(
                    TextView::new("5s")
                        .style(Style::highlight())
                        .h_align(cursive::align::HAlign::Right)
                        .with_name("time")
                        .min_width(3),
                ),
        ),
    );

    runner.refresh();

    let start = Instant::now();

    // Keep it running until
    while runner.is_running() && start.elapsed() < Duration::from_secs(5) {
        let seconds = start.elapsed();
        let time = Duration::from_secs(5).saturating_sub(seconds);
        runner.call_on_name("time", |view: &mut TextView| {
            view.set_content(format!("{}s", time.as_secs()))
        });
        runner.refresh();
        runner.step();
    }

    runner.quit();
}
