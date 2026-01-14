use gh_tui::App;

fn main() -> anyhow::Result<()> {
    // runs closure, providing a terminal instance
    // once closed, terminal is cleaned up
    // then we can return any errors and they will be seen without leftover tui
    ratatui::run(|terminal| App::default().run(terminal))?;
    Ok(())
}
