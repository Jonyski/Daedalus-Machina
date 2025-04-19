use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Widget},
    prelude::*,
    DefaultTerminal, Frame,
};
use std::io;

#[derive(Default)]
enum MachinaState {
	#[default]
	MAIN_MENU,
	COMMAND_LINE
}

// The Machina is equivalent to the "App" struct in other ratatui projects
#[derive(Default)]
pub struct Machina {
	should_exit: bool, // when true, the program terminates
	state: MachinaState // the context in which the master is immersed
}

impl Machina {
	pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
		while !self.should_exit {
			terminal.draw(|frame| self.draw(frame))?;
			self.handle_events()?;
		}
		Ok(())
	}

	fn draw(&mut self, frame: &mut Frame) {
		let p = Paragraph::new("Daedalus Machina");
		frame.render_widget(p.block(Block::new().borders(Borders::ALL)), frame.area());
	}

	fn handle_events(&mut self) -> io::Result<()> {
		Ok(())
	}

}

impl Widget for &Machina {
	fn render(self, area: Rect, buf: &mut Buffer) {

	}
}