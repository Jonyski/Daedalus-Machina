use crate::daedalus::{self, colors};
use ratatui::{
    style::{Style},
    text::{Line},
    widgets::{Paragraph, Block, Borders, BorderType, Padding, Wrap},
    layout::{Layout, Direction, Constraint},
    prelude::Rect,
    Frame
};

fn get_outer_layout() -> Layout {
    Layout::default()
           .direction(Direction::Horizontal)
           .constraints([
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Length(1)
            ])
}

fn get_inner_layout() -> Layout {
    Layout::default()
           .direction(Direction::Vertical)
           .constraints([
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Length(3)
            ])
}

fn get_sheet_info<'a>() -> Paragraph<'a> {
    let name = Line::raw("---NOME DA SILVA SAURO---");
    let age = Line::raw("idade: 20");
    let class = Line::raw("classe: Humano");
    let description = Line::raw("descrição: O humano mais normal de todos os tempos, ele não tem nenhuma característica que o destaca além de sua mediocridade.");
    let hp = Line::raw("🫀 vida: 16/20");
    let mana = Line::raw("⭐ mana: 100/100");
    let sanity = Line::raw("🧠 sanidade: 4/10");
    let luck = Line::raw("🎲 sorte: 5");

    Paragraph::new(vec![name,
                        age,
                        class,
                        description,
                        hp,
                        mana,
                        sanity,
                        luck])
              .wrap(Wrap { trim: true })
              .block(Block::new().padding(Padding::uniform(2)))
}

fn get_inventory<'a>() -> Paragraph<'a> {
    Paragraph::new("- Panela (1x)\n- Pistola (2x)\n- Pé de cabra (1x)\n- Machado (1x)")
              .wrap(Wrap { trim: true })
              .block(Block::new().borders(Borders::ALL).border_type(BorderType::Rounded))
              .style(Style::default().fg(colors::PURPLE))
}

fn get_notes<'a>() -> Paragraph<'a> {
    Paragraph::new("Este jogador está dormindo e envenenado e confuso e bêbado e olhando para um galináceo")
              .wrap(Wrap { trim: true })
              .block(Block::new().borders(Borders::ALL).border_type(BorderType::Rounded))
              .style(Style::default().fg(colors::PURPLE))
}

pub fn draw(frame: &mut Frame, enclosure: Rect) {
    let outer_layout = get_outer_layout().split(enclosure);
    let inner_layout = get_inner_layout().split(outer_layout[2]);
    let container = daedalus::get_container(colors::PURPLE);
    let sheet_info = get_sheet_info();
    let inventory = get_inventory();
    let notes = get_notes();

    frame.render_widget(container, enclosure);
    frame.render_widget(sheet_info, outer_layout[1]);
    frame.render_widget(inventory, inner_layout[1]);
    frame.render_widget(notes, inner_layout[3]);
}