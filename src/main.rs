#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate anyhow;

mod commands;
mod session;

use anyhow::Result;
use clap::SubCommand;

#[tokio::main]
async fn main() -> Result<()> {
    let app = app_from_crate!()
        .subcommand(
            SubCommand::with_name("login")
                .about("authenticates and saves the session details")
                .args_from_usage("-U, --user=[USERNAME] 'Specifies the username to log in with'"),
        )
        .subcommand(SubCommand::with_name("logout").about("ends the current session"))
        .subcommand(SubCommand::with_name("status").about("displays current session status"))
        .subcommand(SubCommand::with_name("list-rooms").about("lists rooms available to the user"))
        .subcommand(
            SubCommand::with_name("alias")
                .about("alias subcommands")
                .subcommand(
                    SubCommand::with_name("add")
                        .about("adds an alias to a room")
                        .args_from_usage(
                            "<ROOM_ID>  'The ID of the room to alias'
                             <ALIAS>    'The new alias to add'",
                        ),
                )
                .subcommand(
                    SubCommand::with_name("delete")
                        .about("deletes an existing alias")
                        .args_from_usage("<ALIAS>    'The alias to delete'"),
                ),
        );
    let matches = app.get_matches();

    match matches.subcommand() {
        ("login", Some(submatches)) => commands::login(submatches.value_of("user")).await?,
        ("status", Some(_)) => commands::status().await?,
        ("list-rooms", Some(_)) => commands::list_rooms().await?,
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
            let mut out = io::stdout();
            app.write_long_help(&mut out).unwrap();
        }
        (c, _) => {
            todo!("Subcommand '{}' not implemented yet!", c);
        }
    }

    Ok(())
}
