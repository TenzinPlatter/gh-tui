use std::{
    env,
    path::{Path, PathBuf},
};

use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Config {
    pub notes_dir: PathBuf,
    pub cache_dir: Option<PathBuf>,
    pub api_token: String,
    pub editor: String,
}

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct ConfigString {
    notes_dir: String,
    cache_dir: Option<String>,
    pub api_token: String,
}

impl Config {
    pub fn read() -> anyhow::Result<Config> {
        let config: ConfigString = confy::load("shortcut-notes", Some("config"))?;

        let notes_dir = {
            let notes_dir: PathBuf = config.notes_dir.into();
            expand_tilde(&notes_dir)
        };

        let cache_dir = config.cache_dir.map(|cache_dir| {
            let cache_dir: PathBuf = cache_dir.into();
            expand_tilde(&cache_dir)
        });

        let editor = env::var("EDITOR").context("$EDITOR is not set")?;

        Ok(Config {
            notes_dir,
            cache_dir,
            api_token: config.api_token,
            editor,
        })
    }

    pub fn write(&self) -> anyhow::Result<()> {
        let config = ConfigString {
            notes_dir: self.notes_dir.to_str().unwrap().to_string(),
            cache_dir: self
                .cache_dir
                .clone()
                .map(|p| p.to_str().unwrap().to_string()),
            api_token: self.api_token.clone(),
        };

        confy::store("shortcut-notes", Some("config"), config).context("Failed to write config")
    }
}

fn expand_tilde(path: &Path) -> PathBuf {
    let path = shellexpand::full(path.to_str().unwrap()).unwrap();
    let path_string = path.to_string();
    let buf = Path::new(&path_string);
    buf.to_path_buf()
}
