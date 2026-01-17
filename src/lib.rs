use ratatui::layout::{Constraint, Direction};

use crate::{
    api::ApiClient,
    pane::ParagraphPane,
    view::{View, ViewBuilder},
};

pub mod api;
pub mod app;
pub mod pane;
pub mod keys;
pub mod view;

pub async fn get_main_view(api_client: ApiClient) -> anyhow::Result<View> {
    let epic_view = api_client.get_epics_view().await?;

    Ok(ViewBuilder::default()
        .add_non_selectable(epic_view)
        .direction(Direction::Vertical)
        .build())
}
