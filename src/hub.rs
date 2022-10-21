use reqwest::blocking::ClientBuilder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    pub url: String,
    pub status: String,
    pub players: Vec<String>,
}

impl World {
    pub fn new() -> Self {
        Self {
            url: String::new(),
            status: String::new(),
            players: Vec::new(),
        }
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hub {
    pub worlds: Vec<World>,
}

impl Hub {
    fn parse_response(body: &str) -> Hub {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum ParsingMode {
            General,
            World,
        }

        #[derive(Debug, Clone)]
        enum ParsingValue {
            String(String),
            Number(i32),
            List(Vec<String>),
            None,
        }

        let lines: Vec<&str> = body.split('\n').collect();
        let mut hub = Hub { worlds: Vec::new() };

        let mut pmode = ParsingMode::General;
        let mut world_data: Option<World> = None;

        let mut end_world_parsing = |world: &mut Option<World>| {
            if let Some(world) = world.take() {
                hub.worlds.push(world);
            } else {
                *world = Some(World::default())
            }
        };

        let parse_line = |line: &str, starts: &str| -> ParsingValue {
            if !line.starts_with(starts) {
                return ParsingValue::None;
            }

            let parts: Vec<&str> = line.split(" = ").collect();
            let value = parts[1].trim_end_matches('\r');

            if value.starts_with("list(") {
                if value == "list()" {
                    return ParsingValue::List(Vec::new());
                }

                let mut list = Vec::new();
                let value = value.trim_start_matches("list(").trim_end_matches("\")");

                value
                    .trim_start_matches("list(")
                    .trim_end_matches("\")")
                    .split(',')
                    .map(|value| value.trim_matches('"').to_string())
                    .for_each(|value| list.push(value));

                ParsingValue::List(list)
            } else if value.starts_with('"') {
                ParsingValue::String(value.trim_matches('"').to_string())
            } else {
                ParsingValue::Number(value.parse().unwrap())
            }
        };

        for line in lines {
            if line.starts_with("world") {
                end_world_parsing(&mut world_data);
                pmode = ParsingMode::World;
                world_data = Some(World::default());
                continue;
            } else if line.starts_with("general") {
                end_world_parsing(&mut world_data);
                pmode = ParsingMode::General;
                continue;
            }

            if pmode != ParsingMode::World {
                continue;
            }

            let mut world_data = world_data.as_mut().unwrap();

            if let ParsingValue::String(url) = parse_line(line, "\turl") {
                world_data.url = url
            } else if let ParsingValue::String(status) = parse_line(line, "\tstatus") {
                world_data.status = status
            } else if let ParsingValue::List(players) = parse_line(line, "\tplayers") {
                world_data.players = players
            }
        }

        hub
    }

    pub fn ss13() -> Hub {
        let client = ClientBuilder::new().build().unwrap();
        let response = client
            .get("http://www.byond.com/games/Exadv1/SpaceStation13?format=text")
            .send()
            .unwrap();

        let body = response.text().unwrap();

        Self::parse_response(&body)
    }
}
