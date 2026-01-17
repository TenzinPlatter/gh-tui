use ratatui::layout::{Constraint, Direction};

use crate::{
    block::{CounterBlock, ParagraphBlock}, view::{View, ViewBuilder}
};

pub mod api;
pub mod app;
pub mod block;
pub mod keys;
pub mod view;

pub async fn get_main_view() -> anyhow::Result<View> {
    if std::env::var("SHORTCUT_API_TOKEN").is_err() {
        return Ok(ViewBuilder::default()
            .add_non_selectable(ParagraphBlock::not_authenticated())
            .build());
    }

    let epics = ParagraphBlock::get_owned_epics().await?;

    Ok(ViewBuilder::default()
        .add_selectable_with_constraint(counters, Constraint::Percentage(80))
        .add_non_selectable_with_constraint(instructions, Constraint::Percentage(20))
        .direction(Direction::Vertical)
        .build())
}
