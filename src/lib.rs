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

use crate::{
    block::{CounterBlock, Selectable},
    keys::AppKey,
};

pub mod block;
pub mod keys;

#[derive(Default, Debug)]
pub struct App {
    counter_blocks: Vec<CounterBlock>,
    exit: bool,
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
            // TODO: handle navigation, other keys fallthrough
            _ => {}
        }

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> anyhow::Result<()> {
        if key_event.code == AppKey::Quit.into() {
            self.exit = true;
        } else {
            for block in &mut self.counter_blocks {
                if block.is_selected() {
                    block.handle_key_event(&key_event);
                }
            }
        }

        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let blocks: Vec<_> = (0..2).map(|_| CounterBlock::default()).collect();

        let (top, bottom) = get_layout(area);

        for (index, block) in blocks.into_iter().enumerate() {
            block.render(top[index], buf);
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
