use matrix_sdk::{
    ruma::{api::client::r0::alias, RoomAliasId, RoomId, UserId},
    Client, Session, SyncSettings,
};
use rpassword::prompt_password_stderr;
use std::convert::TryFrom;
use std::io::{self, Write};

use crate::util::{build_client_config, restore_session, save_session};

type CommandResult = Result<(), Box<dyn std::error::Error>>;

macro_rules! room_fmt {() => {"{:30.30} | {:30.30} | {}"};}

pub async fn login(username: Option<&str>) -> CommandResult {
    let user_id = match username {
        Some(s) => UserId::try_from(s)?,
        None => {
            let mut s = String::new();

            eprint!("Username: ");
            io::stderr().flush().unwrap();

            let stdin = io::stdin();
            stdin.read_line(&mut s)?;
            UserId::try_from(s.trim_end())?
        }
    };

    let password = prompt_password_stderr("Password: ")?;

    let client =
        Client::new_from_user_id_with_config(user_id.clone(), build_client_config()).await?;
    let response = client
        .login(user_id.localpart(), &password, None, Some("mxadm"))
        .await?;
    println!(
        "{} logged in? {}",
        client.homeserver().await,
        client.logged_in().await
    );

    let session_info = Session {
        access_token: response.access_token,
        user_id: response.user_id,
        device_id: response.device_id,
    };
    save_session(session_info)?;
    Ok(())
}

pub async fn status() -> CommandResult {
    let client = restore_session().await?;

    // Need to make a query requiring a valid token to check session validity
    match client.devices().await {
        Ok(_) => {
            println!(
                "Logged in as {} ({}) with device ID {}",
                client.user_id().await.unwrap(),
                client.display_name().await?.unwrap(),
                client.device_id().await.unwrap()
            );
        }
        Err(e) => println!("Not logged in: {:?}", e),
    }

    Ok(())
}

pub async fn list_rooms() -> CommandResult {
    let client = restore_session().await?;

    println!("Syncing...");
    // client.sync_once(SyncSettings::default()).await?;
    println!(" ...done");

    println!("Joined rooms:");
    println!(room_fmt!(), "Name", "Main alias", "Room ID");
    println!(room_fmt!(), "----", "----------", "-------");
    for r in client.joined_rooms() {
        println!(
            room_fmt!(),
            r.display_name().await?,
            r.canonical_alias().map_or("".into(), |a| a.into_string()),
            r.room_id().as_str(),
        );
    }

    Ok(())
}

pub async fn add_alias(room_id: &str, alias: &str) -> CommandResult {
    let room_id = RoomId::try_from(room_id)?;
    let alias_id = RoomAliasId::try_from(alias)?;
    let client = restore_session().await?;


    todo!()
}