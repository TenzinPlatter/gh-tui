use crate::api::{API_BASE_URL, API_TOKEN};

pub async fn get_owned_epics() -> anyhow::Result<Vec<String>> {
    let response = reqwest::Client::new()
        .get(format!("{}/epics", API_BASE_URL))
        .header("Shortcut-Token", API_TOKEN.as_str())
        .json(serde_json::json!({
            "includes_description": false
        }))
        .send()
        .await?;

    todo!()
}
