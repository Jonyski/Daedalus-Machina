mod menu;
mod nav;
mod command_line;
pub mod colors;

use command_line::CommandLine as cl;
use crate::character_sheets as cs;
use ratatui::crossterm::event::{KeyEvent, KeyCode, KeyModifiers, MouseEvent};
use ratatui::{
    widgets::{Block, Borders, Paragraph, BorderType},
    style::{Color},
    layout::{Layout, Constraint, Flex, Rect},
    prelude::Direction,
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
    context: Context,           // what is the master doing currently
    active_module: Module,      // what module is the main one being used
    last_key: Option<KeyEvent>, // last key pressed (may be useless???)
    pub command_line: cl<'a>,   // the command line
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
        // initializing the layout
        let daedalus_layout = get_daedalus_layout().split(frame.area());
        // draw the navigation tabs
        nav::draw(frame, daedalus_layout[0]);
        // draw the active module
        match self.active_module {
            Module::MENU => menu::draw(frame, daedalus_layout[1]),
            Module::SHEET => cs::draw(frame, daedalus_layout[1]),
            _ => {}
        }
        // draw the command line
        self.command_line.draw(frame, daedalus_layout[1]);
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) -> io::Result<()> {
        let mut result = io::Result::Ok(());
        if self.command_line.active {
            result = self.command_line.handle_key_event(key);
        } 
        match key.code {
            // esc ativa a linha de comando
            KeyCode::Esc => self.command_line.active = !self.command_line.active,
            // as teclas F mudam de aba
            KeyCode::F(1) => {self.active_module = Module::SHEET;},
            KeyCode::F(2) => {self.active_module = Module::DICE;},
            KeyCode::F(3) => {self.active_module = Module::ITEM;},
            KeyCode::F(4) => {self.active_module = Module::NOTES;},
            KeyCode::F(5) => {self.active_module = Module::WORLD;},
            KeyCode::F(6) => {self.active_module = Module::MENU;},
            _ => {result = Ok(());}
        }
        self.last_key = Some(key);
        result
    }

    pub fn handle_mouse_event(&mut self, mouse_event: MouseEvent) -> io::Result<()> {
        if self.command_line.mouse_hit(mouse_event) {
            self.command_line.handle_mouse_event(mouse_event);
        } //else self.mouse_hit_nav(mouse_event) {
            //nav::handle_mouse_event(mouse_event);
        //}
        Ok(())
    }

    #[allow(dead_code)]
    pub fn get_last_key(& self) -> KeyEvent {
        if self.last_key != None {
            return self.last_key.expect("REASON")
        }
        KeyEvent::new(KeyCode::Null, KeyModifiers::NONE)
    }
}

fn get_daedalus_layout() -> Layout {
    // 10% navigation and 90% content
    Layout::default()
           .direction(Direction::Horizontal)
           .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(90)
           ])
}

#[allow(dead_code)]
pub fn center_horizontal(area: Rect, width: u16) -> Rect {
    // generates an area in which a widget gets centered in a container horizontally
    let [area] = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .areas(area);
    area
}

pub fn center_vertical(area: Rect, height: u16) -> Rect {
    // generates an area in which a widget gets centered in a container vertically
    let [area] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(area);
    area
}

pub fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    // generates an area in which a widget gets centered in a container
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}

pub fn get_container<'a>(color: Color) -> Paragraph<'a> {
    // generates an empty box (paragraph) with borders of a certain color
    Paragraph::new("")
              .block(Block::new().borders(Borders::ALL).border_type(BorderType::Rounded))
              .style(color)
}