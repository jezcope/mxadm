#[macro_use]
extern crate clap;

mod commands;

use clap::{App, SubCommand};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .args_from_usage(
            "-c, --config=[FILE] 'Sets custom config file'
             -v, --verbose       'Prints more details when running'
             -T, --token=[TOKEN] 'Specifies token for authentication'",
        )
        .subcommand(
            SubCommand::with_name("login")
                .about("authenticates and saves the session details")
                .args_from_usage("-U, --user=[USERNAME] 'Specifies the username to log in with'"),
        )
        .subcommand(SubCommand::with_name("logout").about("ends the current session"))
        .subcommand(SubCommand::with_name("status").about("displays current session status"))
        .subcommand(SubCommand::with_name("list-rooms").about("lists rooms available to the user"))
        .get_matches();

    match matches.subcommand() {
        ("login", Some(login_matches)) => commands::login(login_matches.value_of("user")).await?,
        ("logout", Some(_)) => println!("logging out..."),
        ("status", Some(_)) => commands::status().await?,
        ("list-rooms", Some(_)) => println!("listing rooms..."),
        ("", None) => println!("No subcommand given"),
        _ => unreachable!(),
    }

    Ok(())
}
