use std::collections::HashSet;

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
        "Eris EN"
    } else if name.starts_with("<b>Foundation-19</b>") {
        "Foundation-19"
    } else if name.starts_with("<b>Core Station</b>") {
        "Core Station"
    } else if name.starts_with("<b>ARMOSTATION</b>") {
        "ARMOSTATION"
    } else {
        name
    };

    normalized.to_string()
}

fn main() {
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
