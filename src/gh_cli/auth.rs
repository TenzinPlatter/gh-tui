use std::{
    env,
    process::{Command, Output},
};

use anyhow::Context;

const GITHUB_CLI_AUTH_ENV_OVERRIDE: &str = "GH_TUI_AUTHENTICATED";

fn run_auth_cmd(args: &[&str]) -> anyhow::Result<Output> {
    Command::new("gh")
        .arg("auth")
        .args(args)
        .output()
        .context(format!(
            "Failed to run gh auth command with args: {:?}",
            args
        ))
}

pub fn is_authenticated() -> anyhow::Result<bool> {
    #[cfg(debug_assertions)]
    if let Ok(val) = env::var(GITHUB_CLI_AUTH_ENV_OVERRIDE) {
        let should_override = match val.to_lowercase().as_str() {
            "1" | "true" => Some(true),
            "0" | "false" => Some(false),
            _ => None,
        };

        if let Some(should_override) = should_override {
            return Ok(should_override);
        }
    }

    cli_is_authenticated()
}

fn cli_is_authenticated() -> anyhow::Result<bool> {
    Ok(matches!(run_auth_cmd(&["status"])?.status.code(), Some(0)))
}
