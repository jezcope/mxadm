#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;

mod commands;
mod session;

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
        .subcommand(
            SubCommand::with_name("add-alias")
                .about("adds an alias to a room")
                .args_from_usage(
                    "<room_id>  'The ID of the room to alias'
                     <alias>    'The new alias to add'",
                ),
        )
        .subcommand(
            SubCommand::with_name("del-alias")
                .about("deletes an existing alias")
                .args_from_usage("<alias>    'The alias to delete'"),
        )
        .get_matches();

    match matches.subcommand() {
        ("login", Some(submatches)) => commands::login(submatches.value_of("user")).await?,
        ("status", Some(_)) => commands::status().await?,
        ("list-rooms", Some(_)) => commands::list_rooms().await?,
        ("add-alias", Some(submatches)) => {
            commands::add_alias(
                submatches.value_of("room_id").unwrap(),
                submatches.value_of("alias").unwrap(),
            )
            .await?
        }
        ("del-alias", Some(submatches)) => {
            commands::del_alias(submatches.value_of("alias").unwrap()).await?
        }
        ("", None) => eprintln!("No subcommand given"),
        (c, _) => {
            todo!("Subcommand '{}' not implemented yet!", c);
        }
    }

    Ok(())
}
