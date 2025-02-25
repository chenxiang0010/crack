use crate::mobaxterm::util::LicenseType;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub mobaxterm: MobaXterm,
    pub jetbrains: JetBrains,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MobaXterm {
    pub username: String,
    pub version: String,
    pub license_type: LicenseType,
    pub count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JetBrains {
    pub licensee_name: String,
    pub assignee_name: String,
    pub expire_at: String,
    pub update_code: bool,
}

impl Config {
    pub fn new() -> anyhow::Result<Self> {
        let config = serde_json::from_str::<Config>(&fs::read_to_string("config.json")?)?;
        Ok(config)
    }
}
