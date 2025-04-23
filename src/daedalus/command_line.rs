use super::colors;
use ratatui::crossterm::event::{KeyEvent, KeyCode, KeyModifiers};
use ratatui::{
    style::{Style, Color},
    widgets::{Block, Borders, BorderType, Paragraph, Padding},
    layout::{Layout, Direction, Constraint},
    Frame,
    prelude::Rect
};
use tui_textarea::{TextArea, CursorMove};
use std::io;

pub struct CommandLine<'a> {
    pub active: bool,
    pub text_area: TextArea<'a>
}

impl CommandLine<'_> {
    pub fn init() -> Self {
        Self {
            active: true,
            text_area: Self::get_command_line()
        }
    }

    fn get_command_line_enclosure_layout() -> Layout {
        Layout::default()
               .direction(Direction::Vertical)
               .constraints([
                  Constraint::Min(0),
                  Constraint::Length(3)
               ])
    }

    fn get_command_line_layout() -> Layout {
        Layout::default()
               .direction(Direction::Horizontal)
               .constraints([
                    Constraint::Percentage(10),
                    Constraint::Min(0),
                    Constraint::Percentage(10)
                ])
    }

    fn get_command_line<'a>() -> TextArea<'a> {
        let block = Block::default()
                          .borders(Borders::ALL)
                          .style(colors::ORANGE)
                          .padding(Padding::left(3));
        let selection_style = Style::default().bg(colors::ORANGE).fg(Color::Black);
        let mut command_line = TextArea::from([String::from("")]);
        command_line.set_block(block);
        command_line.set_selection_style(selection_style);
        command_line.set_cursor_line_style(Style::default());
        command_line
    }

    pub fn handle_event(&mut self, key: KeyEvent) -> io::Result<()> {
        let result;
        if (self.text_area.selection_range() == None)
           && (key.modifiers.bits() == KeyModifiers::CONTROL.bits() | KeyModifiers::SHIFT.bits()
           || key.modifiers == KeyModifiers::SHIFT) {
            self.text_area.start_selection();
        }

        if key.modifiers !=  KeyModifiers::CONTROL
           && key.modifiers.bits() != KeyModifiers::CONTROL.bits() | KeyModifiers::SHIFT.bits() {
            result = self.handle_text_event(key);
        } else {
            result = self.handle_shortcut_event(key);
        }

        if key.modifiers != KeyModifiers::SHIFT 
           && (key.modifiers.bits() != KeyModifiers::CONTROL.bits() | KeyModifiers::SHIFT.bits())
           && (key.modifiers != KeyModifiers::CONTROL && key.code != KeyCode::Char('a')) {
            self.text_area.cancel_selection();
        }

        result
    }

    fn handle_text_event(&mut self, key: KeyEvent) -> io::Result<()> {
        match key.code {
            KeyCode::Char(_) => {self.text_area.input(key);},
            KeyCode::Enter => {
                self.text_area.delete_line_by_end();
                self.text_area.delete_line_by_head();
            },
            KeyCode::Backspace => {self.text_area.delete_char();},
            KeyCode::Left => {self.text_area.move_cursor(CursorMove::Back);},
            KeyCode::Right => {self.text_area.move_cursor(CursorMove::Forward);},
            KeyCode::Esc => self.active = false,
            _ => return Ok(())
        }
        Ok(())
    }

    fn handle_shortcut_event(&mut self, key: KeyEvent) -> io::Result<()> {
        match key.code {
            KeyCode::Char('c') => {self.text_area.copy();},
            KeyCode::Char('x') => {self.text_area.cut();},
            KeyCode::Char('v') => {self.text_area.paste();},
            KeyCode::Char('z') => {self.text_area.undo();},
            KeyCode::Char('y') => {self.text_area.redo();},
            KeyCode::Char('a') => {self.text_area.select_all();},
            KeyCode::Char('d') => {self.text_area.cancel_selection();},
            KeyCode::Char('p') => {
                self.text_area.move_cursor(CursorMove::WordBack);
                self.text_area.delete_next_word();
            }
            KeyCode::Delete => {self.text_area.delete_next_word();},
            KeyCode::Left => {self.text_area.move_cursor(CursorMove::WordBack);},
            KeyCode::Right => {self.text_area.move_cursor(CursorMove::WordForward);},
            _ => return Ok(())
        }
        Ok(())
    }

    fn update_style(&mut self) {
        if self.active {
            let block = Block::default()
                              .borders(Borders::ALL)
                              .style(colors::ORANGE)
                              .border_type(BorderType::Rounded)
                              .padding(Padding::left(3));
            let style = Style::default().fg(colors::ORANGE);
            let selection_style = Style::default().bg(colors::ORANGE).fg(Color::Black);
            self.text_area.set_block(block);
            self.text_area.set_style(style);
            self.text_area.set_selection_style(selection_style);
        } else {
            let block = Block::default()
                              .borders(Borders::ALL)
                              .style(colors::LIGHT_YELLOW)
                              .border_type(BorderType::Rounded)
                              .padding(Padding::left(3));
            let style = Style::default().fg(colors::LIGHT_YELLOW);
            let selection_style = Style::default().bg(colors::LIGHT_YELLOW).fg(Color::Black);
            self.text_area.set_block(block);
            self.text_area.set_style(style);
            self.text_area.set_selection_style(selection_style);
        }
    }

    fn get_icon<'a>() -> Paragraph<'a> {
        Paragraph::new("  >")
    }

    pub fn draw(&mut self, frame: &mut Frame, enclosure: Rect) {
        let enclosure_layout = CommandLine::<'_>::get_command_line_enclosure_layout().split(enclosure);
        let layout = CommandLine::<'_>::get_command_line_layout().split(enclosure_layout[1]);
        self.update_style();
        frame.render_widget(CommandLine::<'_>::get_icon(), super::center_vertical(layout[1], 1)) ;
        frame.render_widget(&self.text_area, layout[1]);
    }
}