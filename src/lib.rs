use anyhow::Context;
use ratatui::{DefaultTerminal, layout::Direction};

use crate::{
    api::ApiClient,
    app::App,
    view::{View, ViewBuilder},
};

pub mod api;
pub mod app;
pub mod keys;
pub mod pane;
pub mod view;

pub async fn get_main_view(api_client: ApiClient) -> anyhow::Result<View> {
    let epic_view = api_client.get_epics_view().await?;

    Ok(ViewBuilder::default()
        .add_non_selectable(epic_view)
        .direction(Direction::Vertical)
        .build())
}

pub async fn get_api_key() -> anyhow::Result<String> {
    std::env::var("SHORTCUT_API_TOKEN").context(
        "Please set the SHORTCUT_API_TOKEN environment variable to authenticate with Shortcut",
    )
}

pub async fn get_user_id() -> anyhow::Result<String> {
    // TODO: maybe fetch this from the API using the token instead of env var
    std::env::var("SHORTCUT_USER_ID").context(
        "Please set the SHORTCUT_USER_ID environment variable to authenticate with Shortcut",
    )
}

pub async fn run(terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
    let mut app = App::init().await?;
    app.main_loop(terminal).await?;

    Ok(())
}
