mod machina;
mod daedalus;
use crate::machina::*;

use std::io;
use ratatui;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let machina_result = Machina::default().run(&mut terminal);
    ratatui::restore();
    machina_result
}
