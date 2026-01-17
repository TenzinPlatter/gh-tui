use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget, WidgetRef},
};

use crate::{
    block::Selectable,
    keys::{AppKey, KeyHandler}, view::{View, ViewBuilder},
};

pub struct ParagraphBlock {
    paragraph: Paragraph<'static>,
    block: Block<'static>,
    is_selected: bool,
}

impl ParagraphBlock {
    pub fn cli_not_installed() -> Self {
        let paragraph = Paragraph::new(Text::from(Line::from(
            " GitHub CLI ('gh') is not installed or not found in PATH. Please install it and ensure it is accessible from your command line. "
                .red()
                .italic(),
        )));

        let block = Block::bordered().border_set(border::THICK);

        Self {
            paragraph,
            block,
            is_selected: false,
        }
    }

    pub fn not_authenticated() -> Self {
        let paragraph = Paragraph::new(Text::from(Line::from(
            " The SHORTCUT_API_TOKEN environment variable is not set. Please set it to your Shortcut API token to authenticate. "
                .red()
                .italic(),
        )));

        let block = Block::bordered().border_set(border::THICK);

        Self {
            paragraph,
            block,
            is_selected: false,
        }
    }

    pub fn instructions() -> Self {
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

        let paragraph = Paragraph::new(Text::from(vec![
            counter_instructions,
            navigation_instructions,
            quit_instructions,
        ]));

        let block = Block::bordered()
            .title(" Instructions ".bold().underlined().into_centered_line())
            .border_set(border::THICK);

        Self {
            paragraph,
            block,
            is_selected: false,
        }
    }
}

impl WidgetRef for ParagraphBlock {
    #[doc = " Draws the current state of the widget in the given buffer. That is the only method required"]
    #[doc = " to implement a custom widget."]
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        self.paragraph
            .clone()
            .block(self.block.clone())
            .centered()
            .render(area, buf);
    }
}

impl KeyHandler for ParagraphBlock {}

impl Selectable for ParagraphBlock {
    fn is_selected(&self) -> bool {
        self.is_selected
    }

    fn select(&mut self) {
        self.is_selected = true;
    }

    fn unselect(&mut self) {
        self.is_selected = false
    }
}
