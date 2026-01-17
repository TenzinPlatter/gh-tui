use serde::Deserialize;
use uuid::Uuid;

use crate::api::ApiClient;

pub mod view;

#[derive(Deserialize)]
pub struct EpicSlim {
    pub id: i32,
    pub owner_ids: Vec<Uuid>,
}

#[derive(Deserialize)]
pub struct Epic {
    pub id: i32,
    pub completed: bool,
    pub description: String,
    pub name: String,
    pub owner_ids: Vec<Uuid>,
    pub started: bool,
}

impl ApiClient {
    pub async fn get_owned_epics(&self) -> anyhow::Result<Vec<Epic>> {
        let body = serde_json::json!({
            "includes_description": false
        });

        let response = self.get_with_body("epics", body).await?;
        let epics_slim = response.json::<Vec<EpicSlim>>().await?;

        // TODO?: maybe add a limit to how many we request to not bash the api
        let mut epics = Vec::new();
        for epic in epics_slim.iter().take(2) {
            let response = self.get(&format!("epics/{}", epic.id)).await?;
            let epic = response.json::<Epic>().await?;
            epics.push(epic);
        }

        Ok(epics)
    }
}
