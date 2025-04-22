use ratatui::{
    widgets::{Block, Borders, Paragraph, BorderType},
    style::{Style, Color},
    layout::{Layout, Direction, Constraint},
    text::{Line}
};
use std::vec;

pub type NavItem = (Paragraph<'static>, Line<'static>);

fn create_nav_item(label: String, color: Color) -> NavItem {
    (Paragraph::new("")
              .centered()
              .block(Block::new().borders(Borders::ALL ^ Borders::RIGHT).border_type(BorderType::Rounded))
              .style(Style::default().fg(color)),
    Line::raw(label).style(Style::default().fg(color)))
}

pub fn get_nav_layout() -> Layout {
    Layout::default()
           .direction(Direction::Vertical)
           .constraints([
            Constraint::Percentage(6),
            Constraint::Percentage(16),
            Constraint::Percentage(2),
            Constraint::Percentage(16),
            Constraint::Percentage(2),
            Constraint::Percentage(16),
            Constraint::Percentage(2),
            Constraint::Percentage(16),
            Constraint::Percentage(2),
            Constraint::Percentage(16),
            Constraint::Percentage(6)
           ])
}

pub fn get_nav() -> Vec<NavItem> {
    vec![create_nav_item(String::from("Fichas"), Color::Rgb(111, 16, 235)),
         create_nav_item(String::from("Dados"), Color::Rgb(111, 99, 242)),
         create_nav_item(String::from("Itens"), Color::Rgb(79, 240, 237)),
         create_nav_item(String::from("Notas"), Color::Rgb(233, 237, 119)),
         create_nav_item(String::from("Mundo"), Color::Rgb(242, 58, 58))]
}