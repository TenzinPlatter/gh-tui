use std::sync::LazyLock;

pub mod epic;

pub const API_BASE_URL: &str = "https://api.app.shortcut.com/v3";
pub static API_TOKEN: LazyLock<String> = LazyLock::new(|| {
    std::env::var("SHORTCUT_API_TOKEN").expect("SHORTCUT_API_TOKEN must be set")
});
