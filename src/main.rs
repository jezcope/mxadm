#![deny(unsafe_code)]
#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate anyhow;

mod commands;
mod session;

use anyhow::Result;
use clap::{Arg, SubCommand};

#[tokio::main]
async fn main() -> Result<()> {
    let app = app_from_crate!()
        .subcommand(
            SubCommand::with_name("login")
                .about("authenticates and saves the session details")
                .args_from_usage("-U, --user=[MXID] 'Specifies the Matrix ID (like @you:example.com) to log in with'"),
        )
        .subcommand(SubCommand::with_name("logout").about("ends the current session"))
        .subcommand(SubCommand::with_name("status").about("displays current session status"))
        .subcommand(
            SubCommand::with_name("room")
                .about("room subcommands")
                .subcommand(
                    SubCommand::with_name("list")
                        .alias("ls")
                        .about("lists rooms available to the user"),
                )
                .subcommand(
                    SubCommand::with_name("tombstone")
                        .about("add a tombstone redirecting one room to another")
                        .args_from_usage(
                            "<OLD_ROOM_ID>  'The ID (like !aBcD:example.com) of the source room'
                             <NEW_ROOM_ID>  'The ID (like !pQrS:example.com) of the target room'",
                        )
                        .arg(
                            Arg::with_name("MSG")
                                .short("m")
                                .long("message")
                                .help("The message to display in the old rom")
                                .default_value("This room has been replaced"),
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name("alias")
                .about("alias subcommands")
                .subcommand(
                    SubCommand::with_name("add")
                        .about("adds an alias to a room")
                        .args_from_usage(
                            "<ROOM_ID>  'The ID (like !aBcD:example.com) of the room to alias'
                             <ALIAS>    'The new alias (like #alias:example.com) to add'",
                        ),
                )
                .subcommand(
                    SubCommand::with_name("delete")
                        .about("deletes an existing alias")
                        .args_from_usage("<ALIAS>    'The alias (like #alias:example.com) to delete'"),
                ),
        );
    let matches = app.get_matches();

    match matches.subcommand() {
        ("login", Some(submatches)) => commands::login(submatches.value_of("user")).await?,
        ("status", Some(_)) => commands::status().await?,
        ("room", Some(room)) => match room.subcommand() {
            ("list", Some(_)) => commands::list_rooms().await?,
            ("tombstone", Some(submatches)) => {
                commands::tombstone_room(
                    submatches.value_of("OLD_ROOM_ID").unwrap(),
                    submatches.value_of("NEW_ROOM_ID").unwrap(),
                    submatches.value_of("MSG").unwrap(),
                )
                .await?
            }
            (c, _) => {
                todo!("Subcommand '{}' not implemented yet!", c);
            }
        },
        ("alias", Some(alias)) => match alias.subcommand() {
            ("add", Some(submatches)) => {
                commands::add_alias(
                    submatches.value_of("ROOM_ID").unwrap(),
                    submatches.value_of("ALIAS").unwrap(),
                )
                .await?
            }
            ("delete", Some(submatches)) => {
                commands::del_alias(submatches.value_of("ALIAS").unwrap()).await?
            }
            (c, _) => {
                todo!("Subcommand '{}' not implemented yet!", c);
            }
        },
        ("", None) => {
            println!("No subcommand given: see `mxadm help` for usage information");
        }
        (c, _) => {
            todo!("Subcommand '{}' not implemented yet!", c);
        }
    }

    Ok(())
}
