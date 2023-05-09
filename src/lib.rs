use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use eyre::Result;
use std::{cell::RefCell, rc::Rc, time::Duration};
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders},
    Terminal,
};

pub struct App;

impl App {
    pub fn new() -> Self {
        Self
    }
}

pub fn start_ui(app: Rc<RefCell<App>>) -> Result<()> {
    crossterm::terminal::enable_raw_mode()?;

    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // loop {
    //     let app = app.borrow();
    //     // render
    //     terminal.draw(|rect| {
    //         let size = rect.size();
    //         let block = Block::default().title("Block").borders(Borders::ALL);
    //         rect.render_widget(block, size);
    //     })?;
    //     // TODO handle inputs here
    // }

    terminal.draw(|rect| {
        let size = rect.size();
        let block = Block::default().title("Block").borders(Borders::ALL);
        rect.render_widget(block, size);
    })?;

    std::thread::sleep(Duration::from_millis(5000));

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
