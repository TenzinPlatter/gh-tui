use shortcut_notes_tui::{app::App, get_main_view};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let view = get_main_view().await?;

    // runs closure, providing a terminal instance once closed, terminal is cleaned up
    // then we can return any errors and they will be seen without leftover tui
    ratatui::run(|terminal| App::from(view).run(terminal))?;
    Ok(())
}
