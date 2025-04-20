                                                          
// ____            _     _            _____         _   _         
//|    \ ___ ___ _| |___| |_ _ ___   |     |___ ___| |_|_|___ ___ 
//|  |  | .'| -_| . | .'| | | |_ -|  | | | | .'|  _|   | |   | -_|
//|____/|__,|___|___|__,|_|___|___|  |_|_|_|__,|___|_|_|_|_|_|___|         
//


// ____                        __            ___                                                  __                            
///\  _`\                     /\ \          /\_ \                        /'\_/`\                 /\ \      __                   
//\ \ \/\ \     __       __   \_\ \     __  \//\ \    __  __    ____    /\      \     __      ___\ \ \___ /\_\    ___      __   
// \ \ \ \ \  /'__`\   /'__`\ /'_` \  /'__`\  \ \ \  /\ \/\ \  /',__\   \ \ \__\ \  /'__`\   /'___\ \  _ `\/\ \ /' _ `\  /'__`\ 
//  \ \ \_\ \/\ \L\.\_/\  __//\ \L\ \/\ \L\.\_ \_\ \_\ \ \_\ \/\__, `\   \ \ \_/\ \/\ \L\.\_/\ \__/\ \ \ \ \ \ \/\ \/\ \/\  __/ 
//   \ \____/\ \__/.\_\ \____\ \___,_\ \__/.\_\/\____\\ \____/\/\____/    \ \_\\ \_\ \__/.\_\ \____\\ \_\ \_\ \_\ \_\ \_\ \____\
//    \/___/  \/__/\/_/\/____/\/__,_ /\/__/\/_/\/____/ \/___/  \/___/      \/_/ \/_/\/__/\/_/\/____/ \/_/\/_/\/_/\/_/\/_/\/____/
//                                                                                                                              


// _____                 _       _               ______              _     _             
//(____ \               | |     | |             |  ___ \            | |   (_)            
// _   \ \ ____  ____ __| | ____| |_   _  ___   | | _ | | ____  ____| |__  _ ____   ____ 
//| |   | / _  |/ ___) _| |/ _  | | | | |/___)  | || || |/ _  |/ ___)  _ \| |  _ \ / _  |
//| |__/ ( ( | (  ___)(_| ( ( | | | |_| |___ |  | || || ( ( | ( (___| | | | | | | ( ( | | 
//|_____/ \_||_|\____)____|\_||_|_|\____(___/   |_||_||_|\_||_|\____)_| |_|_|_| |_|\_||_|
                                                                                       
mod menu;

use ratatui::{
    widgets::{Block, Borders, Paragraph},
    style::{Style, Color},
    layout::{Layout, Direction, Constraint},
    prelude::Rect,
    Frame,
};
use std::rc::Rc;
use std::io;

// The context in which the dungeon master is immersed
#[allow(non_camel_case_types)]
#[derive(Default)]
#[derive(PartialEq)]
enum Context {
    #[default]
    MAIN_MENU
}

// The current active module (or page) in the machine
#[allow(non_camel_case_types)]
#[derive(Default)]
#[derive(PartialEq)]
enum Module {
    #[default]
    MENU
}

#[derive(Default)]
pub struct Daedalus {
    context: Context,
    active_module: Module
}

impl Daedalus {
    pub fn draw(&mut self, frame: &mut Frame) {
        let daedalus_layout = menu::get_daedalus_layout().split(frame.area());;
        let nav_layout = menu::get_nav_layout().split(daedalus_layout[0]);
        let command_line_enclosure_layout = get_command_line_enclosure_layout().split(daedalus_layout[1]);
        let command_line_layout = get_command_line_layout().split(command_line_enclosure_layout[1]);
        let command_line = get_command_line(Color::Rgb(255, 225, 150));

        if self.active_module == Module::MENU {
            let menu_layout = menu::get_menu_layout().split(daedalus_layout[1]);
            let container = get_container(Color::Rgb(255, 225, 150));
            let title = menu::get_title();
            let subtitle = menu::get_subtitle();
            let help = menu::get_help();
            frame.render_widget(container, daedalus_layout[1]);
            frame.render_widget(title, menu_layout[1]);
            frame.render_widget(subtitle, menu_layout[2]);
            frame.render_widget(help, menu_layout[3]);
        }

        frame.render_widget(command_line, command_line_layout[1]);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        Ok(())
    }

}


pub fn get_container<'a>(color: Color) -> Paragraph<'a> {
    Paragraph::new("")
              .block(Block::new().borders(Borders::ALL))
              .style(Style::default().fg(color))
}

pub fn get_command_line_enclosure_layout() -> Layout {
    Layout::default()
           .direction(Direction::Vertical)
           .constraints([
              Constraint::Min(0),
              Constraint::Length(3)
           ])
}

pub fn get_command_line_layout() -> Layout {
    Layout::default()
           .direction(Direction::Horizontal)
           .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3)
            ])
}

pub fn get_command_line<'a>(color: Color) -> Paragraph<'a> {
    Paragraph::new(">")
              .block(Block::new().borders(Borders::ALL))
              .style(Style::default().fg(color))
}