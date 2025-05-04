use ratatui::crossterm::{self, event::{
    self,
    Event,
    KeyCode, 
    KeyEvent, 
    KeyEventKind, 
    EnableMouseCapture, 
    DisableMouseCapture, 
    MouseEvent, 
    MouseEventKind, 
    MouseButton}};
use ratatui::{DefaultTerminal};
use std::io;
use crate::daedalus::*;

// The Machina is equivalent to the "App" struct in other ratatui projects
pub struct Machina<'a> {
    should_exit: bool,      // when true, the program terminates
    daedalus: Daedalus<'a>, // the daedalus instance associated with this program
}

impl Machina<'_> {
    pub fn init() -> Self {
        Self {should_exit: false,
              daedalus: Daedalus::init()}
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        // boilerplate stuff
        crossterm::execute!(std::io::stdout(), EnableMouseCapture)?; // enables mouse events
        crossterm::terminal::enable_raw_mode().expect("raw mode enable");
        while !self.should_exit {
            // drawing tasks are delegated to Daedalus
            terminal.draw(|frame| self.daedalus.draw(frame))?;
            self.handle_events()?;
        }
        // cleaning up to avoid side effects on the terminal section
        crossterm::execute!(std::io::stdout(), DisableMouseCapture)?;
        crossterm::terminal::disable_raw_mode().expect("raw mode disabled");
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        // key and mouse events are redirected to their own handlers
        let mut result = Ok(());
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                result = self.handle_key_event(key_event)
            }
            Event::Mouse(mouse_event) if mouse_event.kind == MouseEventKind::Down(MouseButton::Left) => {
                result = self.handle_mouse_event(mouse_event)
            }
            _ => {}
        };
        result
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> io::Result<()> {
        // if the command line is active, machina-level events shouldn't run
        // and thus are redirected to Daedalus's handler
        if self.daedalus.command_line.active {
            self.daedalus.handle_key_event(key_event).expect("key event not handled");
        } else {
            match key_event.code {
                KeyCode::Char('q') => self.exit(), // "q" quits the program
                _ => self.daedalus.handle_key_event(key_event).expect("daedalus event not handled")
            }
        }
        Ok(())
    }

    fn handle_mouse_event(&mut self, mouse_event: MouseEvent) -> io::Result<()> {
        // th Machina doesn't have any mouse events, so all of them are redirected to Daedalus
        self.daedalus.handle_mouse_event(mouse_event)
    }

    fn exit(&mut self) {
        self.should_exit = true;
    }

}