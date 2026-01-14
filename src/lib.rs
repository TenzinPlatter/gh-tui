pub mod keys;

use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

use crate::keys::AppKey;

#[derive(Default, Debug)]
pub struct App {
    counters: [u8; 2],
    exit: bool,
    err_msgs: [Option<String>; 2],
    selected: usize,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?; // blocks until an event occurs, thus only draw on change
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> anyhow::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)?;
            }
            _ => {}
        }

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> anyhow::Result<()> {
        if key_event.code == AppKey::Left.into() {
            // alternate between 0 and 1
            self.selected = if self.selected == 0 { 1 } else { 0 };
        } else if key_event.code == AppKey::Right.into() {
            self.selected = if self.selected == 1 { 0 } else { 1 };
        } else if key_event.code == AppKey::Up.into() {
            // increment selected counter
            if self.counters[self.selected] == 9 {
                self.err_msgs[self.selected] = Some("Can't go to double digits".into());
            } else {
                self.counters[self.selected] += 1;
                self.err_msgs[self.selected] = None;
            }
        } else if key_event.code == AppKey::Down.into() {
            // decrement selected counter
            if self.counters[self.selected] == 0 {
                self.err_msgs[self.selected] = Some("Can't go below zero".into());
            } else {
                self.counters[self.selected] -= 1;
                self.err_msgs[self.selected] = None;
            }
        } else if key_event.code == AppKey::Quit.into() {
            self.exit = true;
        }

        Ok(())
    }

    fn create_counter_block(&'_ self, index: usize) -> (Block<'_>, Text<'_>) {
        let title = Line::from(" Counter App Tutorial ".bold());

        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK)
            .border_style(if self.selected == index {
                ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)
            } else {
                ratatui::style::Style::default()
            });

        let counter_line = Line::from(vec![
            "Value: ".into(),
            self.counters[index].to_string().yellow().bold(),
        ]);

        let text = if let Some(error) = &self.err_msgs[index] {
            let mut lines = vec![counter_line];
            lines.push(Line::from(""));
            lines.push(Line::from(vec![" Error: ".red().bold(), error.into()]));
            Text::from(lines)
        } else {
            Text::from(counter_line)
        };

        (block, text)
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let blocks: Vec<_> = (0..2).map(|i| self.create_counter_block(i)).collect();

        let (top, bottom) = get_layout(area);

        for (index, (block, text)) in blocks.into_iter().enumerate() {
            Paragraph::new(text)
                .centered()
                .block(block)
                .render(top[index], buf);
        }

        instructions()
            .block(
                Block::bordered()
                    .border_set(border::THICK)
                    .border_style(Style::default()),
            )
            .render(bottom[1], buf);
    }
}

fn get_layout(area: Rect) -> (Vec<Rect>, Vec<Rect>) {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)])
        .split(area);

    let top = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(vertical[0]);

    (top.to_vec(), vertical.to_vec())
}

fn instructions() -> Paragraph<'static> {
    // returns static lifetime as the &str.into() calls wrap a Span<'a> around the &str's lifetime
    // which is 'static
    let counter_instructions = Line::from(vec![
        " Decrement: ".into(),
        AppKey::Up.to_string().blue().bold(),
        " Increment: ".into(),
        AppKey::Down.to_string().blue().bold(),
    ]);

    let navigation_instructions = Line::from(vec![
        " Left: ".into(),
        AppKey::Left.to_string().blue().bold(),
        " Right: ".into(),
        AppKey::Right.to_string().blue().bold(),
    ]);

    let quit_instructions = Line::from(vec![" Quit: ".into(), "<Q> ".blue().bold()]);

    Paragraph::new(Text::from(vec![
        counter_instructions,
        navigation_instructions,
        quit_instructions,
    ]))
    .centered()
}
