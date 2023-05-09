use crossterm::event::{KeyCode, KeyEvent};

pub mod input;
pub mod ui;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AppReturn {
    Continue,
    Quit,
}

pub struct App;

impl App {
    pub fn new() -> Self {
        Self
    }

    pub fn do_action(&self, key: KeyEvent) -> AppReturn {
        if key.code == KeyCode::Char('q') {
            return AppReturn::Quit;
        }

        AppReturn::Continue
    }

    pub fn update_on_tick(&self) -> AppReturn {
        AppReturn::Continue
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
