use crossterm::event::KeyEvent;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

use crate::keys::AppKey;

pub trait Selectable {
    fn is_selected(&self) -> bool;
    fn select(&mut self);
    fn unselect(&mut self);
}

#[derive(Default, Debug)]
pub struct CounterBlock {
    selected: bool,
    counter: u8,
    err_msg: Option<String>,
}

impl CounterBlock {
    pub fn handle_key_event(&mut self, event: &KeyEvent) {
        match event.code.into() {
            AppKey::Up => {
                if self.counter == 9 {
                    self.err_msg = Some("Can't go to double digits".into());
                } else {
                    self.counter += 1;
                    self.err_msg = None;
                }
            }

            AppKey::Down => {
                if self.counter == 0 {
                    self.err_msg = Some("Can't go below zero".into());
                } else {
                    self.counter -= 1;
                    self.err_msg = None;
                }
            }
            _ => {}
        }
    }
}

impl Selectable for CounterBlock {
    fn is_selected(&self) -> bool {
        self.selected
    }

    fn select(&mut self) {
        self.selected = true;
    }

    fn unselect(&mut self) {
        self.selected = false;
    }
}

impl Widget for CounterBlock {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = Line::from(" Counter App Tutorial ".bold());

        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK)
            .border_style(if self.selected {
                Style::default().fg(ratatui::style::Color::Yellow)
            } else {
                Style::default()
            });

        let counter_line = Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow().bold(),
        ]);

        let text = if let Some(error) = &self.err_msg {
            let mut lines = vec![counter_line];
            lines.push(Line::from(""));
            lines.push(Line::from(vec![" Error: ".red().bold(), error.into()]));
            Text::from(lines)
        } else {
            Text::from(counter_line)
        };

        Paragraph::new(text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
