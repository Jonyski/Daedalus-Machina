                                                          
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
mod nav;
mod command_line;

use command_line::CommandLine as cl;
use ratatui::crossterm::event::{self, KeyEvent, KeyCode, KeyModifiers};
use ratatui::{
    widgets::{Block, Borders, Paragraph, BorderType},
    style::{Style, Color},
    layout::{Layout, Constraint, Flex, Rect},
    Frame,
};
use std::io;
use tui_textarea::TextArea;

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

pub struct Daedalus<'a> {
    context: Context,
    active_module: Module,
    last_key: Option<KeyEvent>,
    pub command_line: cl<'a>,
    nav: Vec<nav::NavItem>
}

impl Daedalus<'_> {
    pub fn init() -> Self {
        Self {
            context: Context::MAIN_MENU,
            active_module: Module::MENU,
            last_key: None,
            command_line: cl::init(),
            nav: nav::get_nav()
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let daedalus_layout = menu::get_daedalus_layout().split(frame.area());
        let nav_layout = nav::get_nav_layout().split(daedalus_layout[0]);
        let nav = nav::get_nav();
        let command_line_enclosure_layout = cl::get_command_line_enclosure_layout().split(daedalus_layout[1]);
        let command_line_layout = cl::get_command_line_layout().split(command_line_enclosure_layout[1]);

        for (i, nav_item) in nav.into_iter().enumerate() {
            let position = center(nav_layout[i*2 + 1], Constraint::Length(nav_item.1.width() as u16), Constraint::Length(1));
            frame.render_widget(nav_item.0, nav_layout[i*2 + 1]);
            frame.render_widget(nav_item.1, position);
        }

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

        // rendering the command line
        self.command_line.update_style();
        frame.render_widget(&self.command_line.text_area, command_line_layout[1]);
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) -> io::Result<()> {
        if self.command_line.active {
            return self.command_line.handle_event(key)
        }
        self.last_key = Some(key);
        Ok(())
    }

    pub fn get_last_key(& self) -> KeyEvent {
        if self.last_key != None {
            return self.last_key.expect("REASON")
        }
        KeyEvent::new(KeyCode::Null, KeyModifiers::NONE)
    }
}

pub fn center_horizontal(area: Rect, width: u16) -> Rect {
    let [area] = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .areas(area);
    area
}

pub fn center_vertical(area: Rect, height: u16) -> Rect {
    let [area] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(area);
    area
}

pub fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}

pub fn get_container<'a>(color: Color) -> Paragraph<'a> {
    Paragraph::new("")
              .block(Block::new().borders(Borders::ALL).border_type(BorderType::Rounded))
              .style(color)
}