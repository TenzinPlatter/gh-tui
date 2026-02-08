use std::{
    env,
    path::{Path, PathBuf},
};

use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Config {
    notes_dir: PathBuf,
    cache_dir: Option<PathBuf>,
    api_token: String,
    editor: String,
}

#[derive(Deserialize, Serialize, Default, Clone)]
struct ConfigFile {
    notes_dir: String,
    cache_dir: Option<String>,
    api_token: String,
    editor: Option<String>,
}

impl Config {
    pub fn read() -> anyhow::Result<Config> {
        let config: ConfigFile = confy::load("shortcut-notes", Some("config"))?;

        let notes_dir = expand_tilde(&PathBuf::from(&config.notes_dir));
        let cache_dir = config.cache_dir.map(|s| expand_tilde(&PathBuf::from(s)));
        let editor = config
            .editor
            .or_else(|| env::var("EDITOR").ok())
            .context("editor not set in config and $EDITOR is not set")?;

        Ok(Config {
            notes_dir,
            cache_dir,
            api_token: config.api_token,
            editor,
        })
    }

    pub fn notes_dir(&self) -> &PathBuf {
        &self.notes_dir
    }

    pub fn cache_dir(&self) -> PathBuf {
        self.cache_dir
            .clone()
            .unwrap_or_else(default_cache_dir)
    }

    pub fn api_token(&self) -> &str {
        &self.api_token
    }

    pub fn editor(&self) -> &str {
        &self.editor
    }

    pub fn write(&self) -> anyhow::Result<()> {
        let config = ConfigFile {
            notes_dir: self.notes_dir.to_str().unwrap().to_string(),
            cache_dir: self.cache_dir.as_ref().map(|p| p.to_str().unwrap().to_string()),
            api_token: self.api_token.clone(),
            editor: Some(self.editor.clone()),
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

fn default_cache_dir() -> PathBuf {
    let mut base = env::home_dir().expect("Couldn't find home dir");
    base.push(".cache");
    base.push("shortcut-notes");
    base
}
