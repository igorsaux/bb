use std::{collections::HashSet, fs};

use bb::{database::DataBase, hub::Hub};

fn normalize_server_name(name: &str) -> String {
    let normalized = if name.starts_with("<b>\\[RU] The Fluffy Frontier") {
        "The Fluffy Frontier"
    } else if name.starts_with("<b>Atom Bomb</b>") {
        "Atom Bomb"
    } else if name.starts_with("<b>\\[SS13.RU] Chaotic Onyx") {
        "Onyx Chaotic"
    } else if name.starts_with("<b>\\[RU] SS220 Paradise") {
        "SS220 Paradise"
    } else if name
        .starts_with("<a href=\\\"https://cm-ss13.com/\\\" rel=\\\"nofollow\\\"><b>CM-SS13 ")
    {
        "CM-SS13"
    } else if name.starts_with("<b>Tau Ceti Classic") {
        "Tau Ceti"
    } else if name.starts_with("<b>\\[SS13.RU] Eos Orbital Station") {
        "Onyx Eos"
    } else if name.starts_with("<b>Voidcrew-LRP</b>") {
        "Voidcrew"
    } else if name.starts_with("<b>Fulpstation</b>") {
        "Fulpstation"
    } else if name.starts_with("SS13.SU] <b>Ð‘eÐ»aÑ ÐœeÑ‡Ñ‚a:</b>") {
        "White"
    } else if name.starts_with("<b>/tg/Station") {
        "/TG/"
    } else if name.starts_with("<b>Aurorastation") {
        "Aurorastation"
    } else if name.starts_with("<b>S.P.L.U.R.T.") {
        "S.P.L.U.R.T."
    } else if name.starts_with(
        "<b><a href='https://discord.ss220.space' rel=\\\"nofollow\\\">\\[RU] SS220 TerraGov",
    ) {
        "SS220 TerraGov"
    } else if name.starts_with("<b>Skyrat SS13") {
        "Skyrat"
    } else if name.starts_with("<b>\\[RU] SS220 Sierra") {
        "SS220 Sierra"
    } else if name.starts_with(
        "<b><a href='https://discord.gg/8FZgaS77bH' rel=\\\"nofollow\\\">Dead Space 13",
    ) {
        "Dead Space 13"
    } else if name.starts_with("<b>\\[RU] BOS") {
        "BOS"
    } else if name.starts_with("<b>\\[RU] SS220 tgstation") {
        "SS220 /TG/station"
    } else if name
        .starts_with("<b><a href=\\\"https://goonhub.com\\\" rel=\\\"nofollow\\\">Goonstation")
    {
        "Goonstation"
    } else if name.starts_with("<b>CEV Eris \\[EN]") {
        "Eris"
    } else if name.starts_with("<b>Foundation-19</b>") {
        "Foundation-19"
    } else if name.starts_with("<b>Core Station</b>") {
        "Core Station"
    } else if name.starts_with("<b>ARMOSTATION</b>") {
        "ARMOSTATION"
    } else if name.starts_with("<b>Paradise Station</b>]") {
        "Paradise"
    } else if name.starts_with("<b>Lobotomy Corporation 13</b>]") {
        "Lobotomy Corporation 13"
    } else if name.starts_with("SS13.SU]<b>\\[RU]Shiptest</b>") {
        "Shiptest RU"
    } else if name.starts_with("<b>Ruwebâ€&nbsp;:") {
        "Ruweb"
    } else if name.starts_with("<b>CEV Eris \\[RU] RP server") {
        "Eris RU"
    } else if name.starts_with("<b>SHIPTEST") {
        "Shiptest"
    } else if name.starts_with(
        "<b><a href='https://discord.gg/2dFpfNE' rel=\\\"nofollow\\\">TerraGov Marine Corps",
    ) {
        "TerraGov"
    } else if name.starts_with("<br>\\n<b>Yogstation 13</b>") {
        "Yogstation 13"
    } else if name.starts_with("<b>Maconha Station 13") {
        "Maconha Station 13"
    } else if name.starts_with("<b>\\[RU] ghostStation</b>") {
        "ghostStation RU"
    } else if name.starts_with("SS13.SU]\\n<center><a href=\\\"https://discord.gg/HQjz7YKRAJ\\\" target=\\\"_blank\\\" rel=\\\"nofollow\\\"><b>Ru Civilization 13") {
		"Civilization 13 RU"
	} else if name.starts_with("<b>BeeStation") {
		"BeeStation"
	} else if name.starts_with("<b>Furbee Station") {
		"Furbee Station"
	} else if name.starts_with("<b>SinguloStation13</b>") {
		"SinguloStation13"
	} else if name.starts_with("<b>\\[RU]Coffeé colony") {
		"Coffeé colony RU"
	} else if name.starts_with("<b>Citadel Station 13") {
		"Citadel Station 13"
	} else if name.starts_with("<b>AetherStation</b>") {
		"AetherStation"
	} else if name.starts_with("<b>Monkestation</b>") || name.starts_with("<b>Metatest Station - Unrobust Chill Barfigth</b> &#8212; <b>Monkey Station") {
		"Monkestation"
	} else if name.starts_with("<b>40K-Eipharius - Warhammer 40k") {
		"40K-Eipharius - Warhammer 40k"
	} else if name.starts_with("<b>Mojave Sun") {
		"Mojave Sun"
	} else if name.starts_with("<b>The Sunset Wasteland</b>") {
		"The Sunset Wasteland"
	} else if name.starts_with("<b>BungalowStation</b>") {
		"BungalowStation"
	} else {
        name
    };

    normalized.to_string()
}

fn csv_export() {
    let db = DataBase::load().unwrap();

    let mut writer = csv::Writer::from_path("export.csv").unwrap();

    for pinfo in &db.players_info {
        if pinfo.visits.is_empty() {
            continue;
        }

        writer.write_field(&pinfo.key).unwrap();

        let reg_date = Hub::reg_date(&pinfo.key);

        writer
            .write_field(reg_date.format("%Y-%m-%d").to_string())
            .unwrap();

        let servers: HashSet<String> = pinfo
            .visits
            .iter()
            .map(|server| normalize_server_name(&server.name))
            .collect();

        let mut servers_text = String::new();

        for (idx, server) in servers.iter().enumerate() {
            servers_text += server;

            if idx != servers.len() - 1 {
                servers_text += ";";
            }
        }

        writer.write_field(servers_text).unwrap();
        writer.write_record(None::<&[u8]>).unwrap();
    }
}

fn json_export() {
    let db = DataBase::load().unwrap();

    let mut info = db.players_info;

    info.iter_mut().for_each(|pinfo| {
        pinfo
            .visits
            .iter_mut()
            .for_each(|visit| visit.name = normalize_server_name(&visit.name))
    });

    let content = serde_json::to_string_pretty(&info).unwrap();
    fs::write("export.json", &content).unwrap();
}

fn main() {
    json_export();
}
