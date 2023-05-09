use crossterm::event::KeyEvent;
use std::sync::mpsc::{channel, Receiver, RecvError, Sender};
use std::{thread, time::Duration};

#[derive(Debug, Clone, Copy)]
pub enum InputEvent {
    Input(KeyEvent),
    Tick,
}

pub struct Events {
    rx: Receiver<InputEvent>,
    _tx: Sender<InputEvent>,
}

impl Events {
    pub fn new(tick_rate: Duration) -> Events {
        let (tx, rx) = channel();

        let event_tx = tx.clone();
        thread::spawn(move || loop {
            if crossterm::event::poll(tick_rate).unwrap() {
                if let crossterm::event::Event::Key(key) = crossterm::event::read().unwrap() {
                    event_tx.send(InputEvent::Input(key)).unwrap();
                }
            }
            event_tx.send(InputEvent::Tick).unwrap();
        });

        Events { rx, _tx: tx }
    }

    pub fn next(&self) -> Result<InputEvent, RecvError> {
        self.rx.recv()
    }
}
