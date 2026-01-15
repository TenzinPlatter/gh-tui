use ratatui::layout::{Constraint, Direction};

use crate::{
    block::{CounterBlock, ParagraphBlock},
    gh_cli::{auth::is_authenticated, cli_installed},
    view::{View, ViewBuilder},
};

pub mod app;
pub mod block;
pub mod gh_cli;
pub mod keys;
pub mod view;

pub fn get_main_view() -> anyhow::Result<View> {
    if !cli_installed() {
        return Ok(ViewBuilder::default()
            .add_non_selectable(ParagraphBlock::cli_not_installed())
            .build());
    }

    if !is_authenticated()? {
        return Ok(ViewBuilder::default()
            .add_non_selectable(ParagraphBlock::not_authenticated())
            .build());
    }

    let counters = ViewBuilder::default()
        .add_selectable(CounterBlock::default())
        .add_selectable(CounterBlock::default())
        .direction(ratatui::layout::Direction::Horizontal)
        .build();

    let instructions = ViewBuilder::default()
        .add_non_selectable(ParagraphBlock::instructions())
        .build();

    Ok(ViewBuilder::default()
        .add_selectable_with_constraint(counters, Constraint::Percentage(80))
        .add_non_selectable_with_constraint(instructions, Constraint::Percentage(20))
        .direction(Direction::Vertical)
        .build())
}
