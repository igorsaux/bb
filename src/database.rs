use std::fs;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerVisit {
    pub name: String,
    pub date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerInfo {
    pub key: String,
    pub visits: Vec<ServerVisit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataBase {
    pub players_info: Vec<PlayerInfo>,
}

impl DataBase {
    pub fn save(&self) {
        fs::write("db.json", serde_json::to_string_pretty(&self).unwrap()).unwrap();
    }

    pub fn load() -> std::io::Result<Self> {
        Ok(serde_json::from_str(&fs::read_to_string("db.json")?).unwrap())
    }
}
