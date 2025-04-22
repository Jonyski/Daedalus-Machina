use super::colors;
use ratatui::{
    widgets::{Block, Borders, Paragraph, BorderType},
    style::{Style, Color},
    layout::{Layout, Direction, Constraint},
    text::{Line},
    Frame,
    prelude::Rect
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
    vec![create_nav_item(String::from("Fichas"), colors::PURPLE),
         create_nav_item(String::from("Dados"), colors::BLUE),
         create_nav_item(String::from("Itens"), colors::CYAN),
         create_nav_item(String::from("Notas"), colors::YELLOW),
         create_nav_item(String::from("Mundo"), colors::RED)]
}

pub fn draw(frame: &mut Frame, container: Rect) {
    let nav = get_nav();
    let layout = get_nav_layout().split(container);
    for (i, nav_item) in nav.into_iter().enumerate() {
        let position = super::center(layout[i*2 + 1], Constraint::Length(nav_item.1.width() as u16), Constraint::Length(1));
        frame.render_widget(nav_item.0, layout[i*2 + 1]);
        frame.render_widget(nav_item.1, position);
    }
}