mod commands;

use std::env;
use commands::get_server::fetch_server;
use commands::eula_updater::eula_updater;
use reqwest::Error;
use clap::{ Arg, Command };

#[tokio::main]
async fn main() -> Result<(), Error> {
    env::set_var("RUST_BACKTRACE", "1");
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
                Arg::new("version")
                .short('v')
                .required(true)
            )
        )
        .subcommand(
            Command::new("eula")
            .about("Accept Eula")
            .disable_version_flag(true)
        )
        .get_matches();

    match matches.subcommand() {
        Some(("get_server", sub_matches)) => {
            let version: &str = sub_matches.get_one::<String>("version").expect("Version not specified");
            println!("Fetching jar from site");
            fetch_server::get_server_jar(&version).await?;
        }
        Some(("eula", _sub_matches)) => {
            println!("Updating Eula");
            eula_updater::update!();
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }

    Ok(())
}
