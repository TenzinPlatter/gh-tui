use std::process::Command;

pub mod auth;

pub fn cli_installed() -> bool {
    match Command::new("gh")
        .arg("--version")
        .output()
    {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}
