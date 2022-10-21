use std::{collections::HashSet, fs};

use bb::{
    database::{DataBase, PlayerInfo, ServerVisit},
    hub::Hub,
};
use chrono::Utc;

fn init_db() -> DataBase {
    let keys: HashSet<String> =
        serde_json::from_str(&fs::read_to_string("keys.json").unwrap()).unwrap();

    let info = keys.into_iter().map(|key| PlayerInfo {
        key,
        visits: Vec::new(),
    });

    let db = DataBase {
        players_info: info.collect(),
    };

    db.save();

    db
}

fn main() {
    let mut db = DataBase::load().unwrap_or_else(|_| init_db());
    let ss13 = Hub::ss13();

    for world in ss13.worlds {
        db.players_info.iter_mut().for_each(|pinfo| {
            if world.players.contains(&pinfo.key) {
                pinfo.visits.push(ServerVisit {
                    name: world.status.clone(),
                    date: Utc::now(),
                })
            }
        });
    }

    db.save()
}
