use anyhow::Result;
use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};
use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

#[derive(Clone, Copy, Debug)]
pub enum Event {
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

#[derive(Debug)]
pub struct EventHandler {
    sender: mpsc::Sender<Event>,
    receiver: mpsc::Receiver<Event>,
    handler: thread::JoinHandle<()>,
}

impl EventHandler {
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel();
        let handler = {
            let sender = sender.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);

                    if event::poll(timeout).expect("No events available") {
                        match event::read().expect("Unable to read event") {
                            CrosstermEvent::Key(e) => sender.send(Event::Key(e)),
                            CrosstermEvent::Mouse(e) => sender.send(Event::Mouse(e)),
                            CrosstermEvent::Resize(w, h) => sender.send(Event::Resize(w, h)),
                            _ => unimplemented(),
                        }
                        .expect("Failed to send terminal event")
                    }

                    if last_tick.elapsed() >= tick_rate {
                        sender.send(Event::Tick).expect("Failed to send tick event");
                        last_tick = Instant::now();
                    }
                }
            })
        };
        Self {
            sender,
            receiver,
            handler,
        }
    }

    pub fn next(&self) -> Result<Event> {
        Ok(self.receiver.recv()?)
    }
}