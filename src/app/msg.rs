use crossterm::event::KeyEvent;

use crate::api::{epic::Epic, iteration::Iteration, story::Story};

/// All possible state transitions in the app
#[derive(Debug, Clone)]
pub enum Msg {
    // Keyboard input
    KeyPressed(KeyEvent),

    // Navigation
    FocusNextPane,
    FocusPrevPane,

    // Pane-specific messages
    StoryList(StoryListMsg),
    Epic(EpicMsg),

    // Background task results (former AppEvents)
    StoriesLoaded { stories: Vec<Story>, from_cache: bool },
    EpicsLoaded(Vec<Epic>),
    IterationLoaded(Iteration),

    // Side effect completions
    NoteOpened,
    CacheWritten,

    // Errors
    Error(String),

    // Application control
    Quit,
}

/// Messages specific to story list pane
#[derive(Debug, Clone, Copy)]
pub enum StoryListMsg {
    SelectNext,
    SelectPrev,
    ToggleExpand,
    OpenNote,
}

/// Messages specific to epic pane (placeholder)
#[derive(Debug, Clone, Copy)]
pub enum EpicMsg {
    // Epic-specific messages will go here
}
