use std::collections::HashSet;

use crate::{
    api::{epic::Epic, iteration::Iteration, story::Story},
    cache::Cache,
    config::Config,
};

/// Root model holding all application state
pub struct Model {
    pub data: DataState,
    pub ui: UiState,
    pub config: Config,
    pub cache: Cache,
}

/// Domain data loaded from API/cache
#[derive(Default)]
pub struct DataState {
    pub stories: Vec<Story>,
    pub epics: Vec<Epic>,
    pub current_iteration: Option<Iteration>,
}

/// UI-specific state (selection, focus, expansion)
#[derive(Default)]
pub struct UiState {
    pub focused_pane: PaneId,
    pub story_list: StoryListState,
    pub epic_pane: EpicPaneState,
}

/// Identifies which pane has focus
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PaneId {
    #[default]
    StoryList,
    Epic,
}

/// Story list pane UI state
#[derive(Default, Clone)]
pub struct StoryListState {
    pub selected_index: Option<usize>,
    pub expanded_items: HashSet<usize>,
}

/// Epic pane UI state (placeholder for now)
#[derive(Default, Clone)]
pub struct EpicPaneState {
    // Epic-specific UI state will go here
}

impl UiState {
    /// Move focus to the next pane (wraps around)
    pub fn focus_next_pane(&mut self) {
        self.focused_pane = match self.focused_pane {
            PaneId::StoryList => PaneId::Epic,
            PaneId::Epic => PaneId::StoryList,
        };
    }

    /// Move focus to the previous pane (wraps around)
    pub fn focus_prev_pane(&mut self) {
        self.focused_pane = match self.focused_pane {
            PaneId::Epic => PaneId::StoryList,
            PaneId::StoryList => PaneId::Epic,
        };
    }
}
