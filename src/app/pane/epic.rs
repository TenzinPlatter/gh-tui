use crossterm::event::KeyEvent;

use crate::app::{cmd::Cmd, msg::EpicMsg};

pub use crate::app::model::EpicPaneState;

pub fn update(
    _state: &mut EpicPaneState,
    _msg: EpicMsg,
) -> Vec<Cmd> {
    vec![Cmd::None]
}

pub fn key_to_msg(_key: KeyEvent) -> Option<EpicMsg> {
    None
}
