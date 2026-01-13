use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

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
        if key_event.code == KeyCode::Left {
            // alternate between 0 and 1
            self.selected = 0;
        } else if key_event.code == KeyCode::Right {
            self.selected = 1;
        } else if key_event.code == KeyCode::Up {
            // increment selected counter
            if self.counters[self.selected] == u8::MAX {
                self.err_msgs[self.selected] = Some("Counter overflow".into());
            } else {
                self.counters[self.selected] += 1;
                self.err_msgs[self.selected] = None;
            }
        } else if key_event.code == KeyCode::Down {
            // decrement selected counter
            if self.counters[self.selected] == u8::MIN {
                self.err_msgs[self.selected] = Some("Counter underflow".into());
            } else {
                self.counters[self.selected] -= 1;
                self.err_msgs[self.selected] = None;
            }
        } else if key_event.code == KeyCode::Char('q') {
            self.exit = true;
        }

        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut blocks = vec![];
        for i in 0..2 {
            let up = if i == 0 { " <Up> " } else { " <W> " };

            let down = if i == 0 { " <Down> " } else { " <S> " };

            let title = Line::from(" Counter App Tutorial ".bold());
            let instructions = Line::from(vec![
                " Decrement ".into(),
                up.blue().bold(),
                " Increment ".into(),
                down.blue().bold(),
                " Quit ".into(),
                "<Q> ".blue().bold(),
            ]);

            let block = Block::bordered()
                .title(title.centered())
                .title_bottom(instructions.centered())
                .border_set(border::THICK)
                .border_style(if self.selected == i {
                    ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)
                } else {
                    ratatui::style::Style::default()
                });

            let counter_line = Line::from(vec![
                "Value: ".into(),
                self.counters[i].to_string().yellow().bold(),
            ]);

            let lines = if let Some(error) = &self.err_msgs[i] {
                let mut lines = vec![counter_line];
                lines.push(Line::from(""));
                lines.push(Line::from(vec![" Error: ".red().bold(), error.into()]));

                Text::from(lines)
            } else {
                Text::from(counter_line)
            };

            blocks.push((block, lines));
        }

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        Paragraph::new(blocks[0].1.clone())
            .centered()
            .block(blocks[0].0.clone())
            .render(chunks[1], buf);

        Paragraph::new(blocks[1].1.clone())
            .centered()
            .block(blocks[1].0.clone())
            .render(chunks[0], buf);
    }
}

fn main() -> anyhow::Result<()> {
    // runs closure, providing a terminal instance
    // once closed, terminal is cleaned up
    // then we can return any errors and they will be seen without leftover tui
    ratatui::run(|terminal| App::default().run(terminal))?;
    Ok(())
}
