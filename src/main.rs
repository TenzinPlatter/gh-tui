use std::env;

use shortcut_notes_tui::{
    api::ApiClient, app::App, pane::ParagraphPane, get_main_view, view::ViewBuilder,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_key = match env::var("API_TOKEN_SHORTCUT") {
        Ok(key) => key,
        Err(_) => {
            let view = ViewBuilder::default()
                .add_non_selectable(ParagraphPane::not_authenticated())
                .build();

            ratatui::run(|terminal| App::from(view).run(terminal))?;

            return Ok(());
        }
    };

    let api_client = ApiClient::new(api_key);
    let view = get_main_view(api_client).await?;

    ratatui::run(|terminal| App::from(view).run(terminal))?;

    Ok(())
}
