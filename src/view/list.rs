use std::cell::Cell;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    symbols::border,
    text::Line,
    widgets::{Block, List, ListItem, ListState, StatefulWidget, WidgetRef},
    style::Style,
};

use crate::{
    keys::{AppKey, KeyHandler},
    pane::Selectable,
};

pub struct ListPane {
    list: List<'static>,
    state: Cell<ListState>,
    is_selected: bool,
}

impl ListPane {
    pub fn new<S>(items: Vec<S>) -> Self
    where
        S: Into<ListItem<'static>>,
    {
        let list_items: Vec<_> = items.into_iter().map(|s| s.into()).collect();

        let block = Block::bordered().border_set(border::THICK);
        let highlighted_symbol: Line = "> ".into();

        let list = List::new(list_items)
            .block(block)
            .highlight_symbol(highlighted_symbol)
            .highlight_style(Style::default().blue().bold())
            .scroll_padding(2); // TODO: calculate this maybe?

        let state = Cell::new(ListState::default());

        ListPane {
            list,
            state,
            is_selected: false,
        }
    }

    pub fn with_state(mut self, state: ListState) -> Self {
        self.state = Cell::new(state);
        self
    }
}

impl WidgetRef for ListPane {
    #[doc = " Draws the current state of the widget in the given buffer. That is the only method required"]
    #[doc = " to implement a custom widget."]
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let mut state = self.state.get();
        (&self.list).render(area, buf, &mut state);
        self.state.set(state);
    }
}

impl Selectable for ListPane {
    fn is_selected(&self) -> bool {
        self.is_selected
    }

    fn select(&mut self) {
        self.is_selected = true;
    }

    fn unselect(&mut self) {
        self.is_selected = false;
    }
}

impl KeyHandler for ListPane {
    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> bool {
        match key_event.code.try_into() {
            // bounds checking etc. is handled by List in render
            Ok(AppKey::Up) => {
                if let Some(curr) = self.state.get().selected() {
                    if curr != 0 {
                        self.state.get_mut().select(Some(curr - 1));
                    } else {
                        let len = self.list.len();
                        self.state.get_mut().select(Some(len - 1));
                    }
                } else {
                    self.state.get_mut().select(Some(0));
                }
            }

            Ok(AppKey::Down) => {
                if let Some(curr) = self.state.get().selected() {
                    let len = self.list.len();
                    if curr != usize::MAX && curr != len - 1 {
                        self.state.get_mut().select(Some(curr + 1));
                    } else {
                        self.state.get_mut().select(Some(0));
                    }
                } else {
                    self.state.get_mut().select(Some(0));
                }
            }

            _ => {
                return false;
            }
        }

        true
    }
}
