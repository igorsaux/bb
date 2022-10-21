use std::{collections::HashSet, fs};

use bb::logs::Logs;

fn main() {
    let mut keys = HashSet::new();

    for dir in fs::read_dir("./logs/").unwrap() {
        let path = dir.unwrap().path();

        if !path.is_dir() {
            continue;
        }

        println!("{}", path.display());
        let logs = Logs::new(path);

        for mut log in logs.files() {
            println!("{}", log.path.display());
            let file_keys = log.keys().clone();
            println!("keys: {}", file_keys.len());
            keys.extend(file_keys);
        }
    }

    println!("saving to keys.json");
    fs::write("keys.json", serde_json::to_string_pretty(&keys).unwrap()).unwrap();
}
