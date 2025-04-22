use super::colors;
use ratatui::{
    style::{Style, Stylize},
    text::{Line},
    widgets::{Paragraph},
    layout::{Layout, Direction, Constraint}
};

pub fn get_daedalus_layout() -> Layout {
    Layout::default()
           .direction(Direction::Horizontal)
           .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(90)
           ])
}

pub fn get_menu_layout() -> Layout {
    Layout::default()
           .direction(Direction::Vertical)
           .constraints([
            Constraint::Length(1),
            Constraint::Length(8),
            Constraint::Length(2),
            Constraint::Min(2),
            Constraint::Length(3)
           ])
}

pub fn get_title<'a>() -> Paragraph<'a> {
    let l1 = Line::raw(" _____                 _       _               ______              _     _             ");
    let l2 = Line::raw("(____ \\               | |     | |             |  ___ \\            | |   (_)            ");
    let l3 = Line::raw(" _   \\ \\ ____  ____ __| | ____| |_   _  ___   | | _ | | ____  ____| |__  _ ____   ____ ");
    let l4 = Line::raw("| |   | / _  |/ ___) _| |/ _  | | | | |/___)  | || || |/ _  |/ ___)  _ \\| |  _ \\ / _  |");
    let l5 = Line::raw("| |__/ ( ( | (  ___)(_| ( ( | | | |_| |___ |  | || || ( ( | ( (___| | | | | | | ( ( | |"); 
    let l6 = Line::raw("|_____/ \\_||_|\\____)____|\\_||_|_|\\____(___/   |_||_||_|\\_||_|\\____)_| |_|_|_| |_|\\_||_|");

    Paragraph::new(vec![l1, l2, l3, l4, l5, l6])
              .bold()
              .centered()
              .style(Style::default().fg(colors::LIGHT_YELLOW))
}

pub fn get_subtitle<'a>() -> Paragraph<'a> {
    Paragraph::new(Line::raw("O GRIMÓRIO INFINITO DOS MESTRES")
                        .bold()
                        .centered()
                        .style(Style::default().fg(colors::ORANGE)))
}

pub fn get_help<'a>() -> Paragraph<'a> {
    let l1 = Line::raw("help").style(Style::default().fg(colors::DISCRETE_RED));
    let l3 = Line::raw("esc: inserir comando    ").style(Style::default().fg(colors::GREY_RED));
    let l4 = Line::raw("f1: fichas de personagem").style(Style::default().fg(colors::GREY_RED));
    let l5 = Line::raw("f2: rolagem de dados    ").style(Style::default().fg(colors::GREY_RED));
    let l6 = Line::raw("f3: catálogo de itens   ").style(Style::default().fg(colors::GREY_RED));
    let l7 = Line::raw("f4: anotações           ").style(Style::default().fg(colors::GREY_RED));
    let l8 = Line::raw("f5: construção de mundo ").style(Style::default().fg(colors::GREY_RED));
    let l2 = Line::raw("q: quit                 ").style(Style::default().fg(colors::GREY_RED));

    Paragraph::new(vec![l1, l2, l3, l4, l5, l6, l7, l8]).centered()
}