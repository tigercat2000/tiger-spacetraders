use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use eyre::Result;
use std::{cell::RefCell, rc::Rc, time::Duration};
use tui::{backend::CrosstermBackend, Terminal};

use crate::app::{
    input::{Events, InputEvent},
    ui, App, AppReturn,
};

pub mod app;

pub fn start_ui(app: Rc<RefCell<App>>) -> Result<()> {
    crossterm::terminal::enable_raw_mode()?;

    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tick_rate = Duration::from_millis(200);
    let events = Events::new(tick_rate);

    loop {
        let app = app.borrow_mut();
        // render
        terminal.draw(|rect| ui::draw(rect, &app))?;
        // handle inputs
        let result = match events.next()? {
            InputEvent::Input(key) => app.do_action(key),
            InputEvent::Tick => app.update_on_tick(),
        };

        if result == AppReturn::Quit {
            break;
        }
    }

    // Restore terminal and close application
    crossterm::terminal::disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
