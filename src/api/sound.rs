use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Sound {
    pub id: String,
    pub name: String,
    pub extension: String,
    pub file_name: String,
    pub file_hash: String,
    pub tags: Vec<String>,
}
