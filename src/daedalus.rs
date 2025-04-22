mod menu;
mod nav;
mod command_line;
mod colors;

use command_line::CommandLine as cl;
use ratatui::crossterm::event::{KeyEvent, KeyCode, KeyModifiers};
use ratatui::{
    widgets::{Block, Borders, Paragraph, BorderType},
    style::{Color},
    layout::{Layout, Constraint, Flex, Rect},
    Frame,
};
use std::io;

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
    MENU,
    SHEET,
    DICE,
    ITEM,
    NOTES,
    WORLD
}

#[allow(dead_code)]
pub struct Daedalus<'a> {
    context: Context,
    active_module: Module,
    last_key: Option<KeyEvent>,
    pub command_line: cl<'a>,
}

impl Daedalus<'_> {
    pub fn init() -> Self {
        Self {
            context: Context::MAIN_MENU,
            active_module: Module::MENU,
            last_key: None,
            command_line: cl::init()
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let daedalus_layout = menu::get_daedalus_layout().split(frame.area());

        nav::draw(frame, daedalus_layout[0]);

        if self.active_module == Module::MENU {
            let menu_layout = menu::get_menu_layout().split(daedalus_layout[1]);
            let container = get_container(colors::LIGHT_YELLOW);
            let title = menu::get_title();
            let subtitle = menu::get_subtitle();
            let help = menu::get_help();
            frame.render_widget(container, daedalus_layout[1]);
            frame.render_widget(title, menu_layout[1]);
            frame.render_widget(subtitle, menu_layout[2]);
            frame.render_widget(help, menu_layout[3]);
        }

        self.command_line.draw(frame, daedalus_layout[1]);
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) -> io::Result<()> {
        let mut result = io::Result::Ok(());
        if self.command_line.active {
            result = self.command_line.handle_event(key);
            match key.code {
                KeyCode::F(1) => {self.active_module = Module::SHEET;},
                KeyCode::F(2) => {self.active_module = Module::DICE;},
                KeyCode::F(3) => {self.active_module = Module::ITEM;},
                KeyCode::F(4) => {self.active_module = Module::NOTES;},
                KeyCode::F(5) => {self.active_module = Module::WORLD;},
                _ => {result = Ok(());}
            }
        } 
        self.last_key = Some(key);
        result
    }

    #[allow(dead_code)]
    pub fn get_last_key(& self) -> KeyEvent {
        if self.last_key != None {
            return self.last_key.expect("REASON")
        }
        KeyEvent::new(KeyCode::Null, KeyModifiers::NONE)
    }
}

#[allow(dead_code)]
pub fn center_horizontal(area: Rect, width: u16) -> Rect {
    let [area] = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .areas(area);
    area
}

#[allow(dead_code)]
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