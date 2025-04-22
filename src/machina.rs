use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal};
use std::io;
use crate::daedalus::*;

// The Machina is equivalent to the "App" struct in other ratatui projects
pub struct Machina<'a> {
    should_exit: bool, // when true, the program terminates
    daedalus: Daedalus<'a>,
}

impl Machina<'_> {
    pub fn init() -> Self {
        Self {should_exit: false,
              daedalus: Daedalus::init()}
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| self.daedalus.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => self.daedalus.handle_key_event(key_event).expect("key event handled")
        }
    }

    fn exit(&mut self) {
        self.should_exit = true;
    }

}