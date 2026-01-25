use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Paragraph, Widget, WidgetRef},
};

use crate::app::model::EpicPaneState;

pub struct EpicView<'a> {
    _state: &'a EpicPaneState,
    _is_focused: bool,
}

impl<'a> EpicView<'a> {
    pub fn new(state: &'a EpicPaneState, is_focused: bool) -> Self {
        Self {
            _state: state,
            _is_focused: is_focused,
        }
    }
}

impl<'a> WidgetRef for EpicView<'a> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().title("Epics");
        let paragraph = Paragraph::new("Epic pane - TODO").block(block);
        Widget::render(paragraph, area, buf);
    }
}
