use std::collections::HashSet;

use crate::{
    api::{epic::Epic, iteration::Iteration, story::Story},
    cache::Cache,
    config::Config,
};

pub struct Model {
    pub data: DataState,
    pub ui: UiState,
    pub config: Config,
    pub cache: Cache,
}

#[derive(Default)]
pub struct DataState {
    pub stories: Vec<Story>,
    pub epics: Vec<Epic>,
    pub current_iteration: Option<Iteration>,
}

#[derive(Default)]
pub struct UiState {
    pub focused_pane: PaneId,
    pub story_list: StoryListState,
    pub epic_pane: EpicPaneState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PaneId {
    #[default]
    StoryList,
    Epic,
}

#[derive(Default, Clone)]
pub struct StoryListState {
    pub selected_index: Option<usize>,
    pub expanded_items: HashSet<usize>,
}

#[derive(Default, Clone)]
pub struct EpicPaneState {
}

impl UiState {
    pub fn focus_next_pane(&mut self) {
        self.focused_pane = match self.focused_pane {
            PaneId::StoryList => PaneId::Epic,
            PaneId::Epic => PaneId::StoryList,
        };
    }

    pub fn focus_prev_pane(&mut self) {
        self.focused_pane = match self.focused_pane {
            PaneId::Epic => PaneId::StoryList,
            PaneId::StoryList => PaneId::Epic,
        };
    }
}
