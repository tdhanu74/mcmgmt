mod commands;

use commands::set_properties::set_properties::{Difficulty, GameMode};
use commands::{get_server::fetch_server, set_properties::set_properties, eula_updater::eula_updater};
use reqwest::Error;
use clap::{ arg, value_parser, Command };

#[tokio::main]
async fn main() -> Result<(), Error> {
    // std::env::set_var("RUST_BACKTRACE", "1");
    let matches = Command::new("bin_cli")
        .version("1.0")
        .bin_name("mcmgmt")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .disable_version_flag(true)
        .propagate_version(true)
        .subcommand(
            Command::new("get_server")
            .about("Fetch server jar from site")
            .disable_version_flag(true)
            .arg(
                arg!(-v --version "version")
                .required(true)
            )
        )
        .subcommand(
            Command::new("eula")
            .about("Accept Eula")
            .disable_version_flag(true)
        )
        .subcommand(
            Command::new("set")
            .about("Set Server Properties")
            .disable_version_flag(true)
            .args([
                arg!(-n --name "Set server name")
                .required(false)
                .value_parser(value_parser!(String)),
            ])
            .args([
                arg!(-d --difficulty "Set difficulty")
                .required(false)
                .value_parser(value_parser!(Difficulty)),
            ])
            .args([
                arg!(-g --gamemode "Set Game Mode")
                .required(false)
                .value_parser(value_parser!(GameMode)),
            ])
            .args([
                arg!(-vd --view-distance "Set View Distance")
                .required(false)
                .value_parser(value_parser!(u8).range(2..32)),
            ])
            .args([
                arg!(-sd --simulation-distance "Set Simulation Distance")
                .required(false)
                .value_parser(value_parser!(u8).range(2..32)),
            ])
            .args([
                arg!(-hc --hardcore "Set Hardcore Mode")
                .required(false)
                .value_parser(value_parser!(bool)),
            ])
            .args([
                arg!(-om --online-mode "Set Online Mode")
                .required(false)
                .value_parser(value_parser!(bool)),
            ])
            .args([
                arg!(-s --seed "Set World Seed")
                .required(false)
                .value_parser(value_parser!(String)),
            ])
        )
        .get_matches();

    match matches.subcommand() {
        Some(("get_server", sub_matches)) => {
            let version: &str = sub_matches.get_one::<String>("version").expect("Version not specified");
            fetch_server::get_server_jar(version).await?;
        }
        Some(("eula", _sub_matches)) => {
            eula_updater::update_eula("eula.txt");
        }
        Some(("set", sub_matches)) => {
            if let Some(name) = sub_matches.get_one::<String>("name") {
                set_properties::set_property("server.properties", "level-name", name.as_str());
            }
            if let Some(diff) = sub_matches.get_one::<Difficulty>("difficulty") {
                set_properties::set_property("server.properties", "difficulty", diff.as_str());
            }
            if let Some(gamemode) = sub_matches.get_one::<GameMode>("gamemode") {
                set_properties::set_property("server.properties", "gamemode", gamemode.as_str());
            }
            if let Some(view_distance) = sub_matches.get_one::<u8>("view-distance") {
                set_properties::set_property("server.properties", "view-distance", view_distance.to_string().as_str());
            }
            if let Some(simulation_distance) = sub_matches.get_one::<u8>("simulation-distance") {
                set_properties::set_property("server.properties","simulation-distance", simulation_distance.to_string().as_str());
            }
            if let Some(hardcore) = sub_matches.get_one::<bool>("hardcore") {
                set_properties::set_property("server.properties", "hardcore", hardcore.to_string().as_str());
            }
            if let Some(online) = sub_matches.get_one::<bool>("online-mode") {
                set_properties::set_property("server.properties", "online-mode", online.to_string().as_str());
            }
            if let Some(seed) = sub_matches.get_one::<String>("seed") {
                set_properties::set_property("server.properties", "level-seed", seed.as_str());
            }
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }

    Ok(())
}
