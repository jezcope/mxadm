use anyhow::{Context, Result};
use matrix_sdk::{
    ruma::{
        api::client::r0::alias,
        events::{room::tombstone::TombstoneEventContent, AnyStateEventContent},
        RoomAliasId, RoomId, UserId,
    },
    Client, Session, SyncSettings,
};
use rpassword::prompt_password_stderr;
use std::convert::TryFrom;
use std::io::{self, Write};

use crate::session::{build_client_config, restore_session, save_session};

type CommandResult = Result<()>;

macro_rules! room_fmt {
    () => {
        "{:30.30} | {:30.30} | {}"
    };
}

pub async fn login(username: Option<&str>) -> CommandResult {
    let user_id = match username {
        Some(s) => {
            UserId::try_from(s).with_context(|| format!("Failed to parse '{}' as User ID", s))?
        }
        None => {
            let mut s = String::new();

            eprint!("Username: ");
            io::stderr().flush().unwrap();

            let stdin = io::stdin();
            stdin.read_line(&mut s)?;
            UserId::try_from(s.trim_end())
                .with_context(|| format!("Failed to parse '{}' as User ID", s))?
        }
    };

    let password = prompt_password_stderr("Password: ")?;

    let client = Client::new_from_user_id_with_config(user_id.clone(), build_client_config())
        .await
        .context("Unable to initialise client")?;
    let response = client
        .login(user_id.localpart(), &password, None, Some("mxadm"))
        .await
        .context("Login failed")?;
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
    let mut sync_settings = SyncSettings::new();
    if let Some(token) = client.sync_token().await {
        sync_settings = sync_settings.token(token);
    }

    print!("Syncing...");
    io::stderr().flush().unwrap();
    client
        .sync_once(sync_settings)
        .await
        .context("Sync failed")?;
    println!(" done");

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

pub async fn tombstone_room(old_room_id: &str, new_room_id: &str, msg: &str) -> CommandResult {
    let old_room_id = RoomId::try_from(old_room_id)
        .with_context(|| format!("Failed to parse '{}' as room ID", old_room_id))?;
    let new_room_id = RoomId::try_from(new_room_id)
        .with_context(|| format!("Failed to parse '{}' as room ID", new_room_id))?;
    let client = restore_session().await?;
    let mut sync_settings = SyncSettings::new();
    if let Some(token) = client.sync_token().await {
        sync_settings = sync_settings.token(token);
    }

    print!("Syncing...");
    io::stderr().flush().unwrap();
    client
        .sync_once(sync_settings)
        .await
        .context("Sync failed")?;
    println!(" done");

    if let Some(old_room) = client.get_joined_room(&old_room_id) {
        let event = TombstoneEventContent::new(msg.to_string(), new_room_id);
        let content = AnyStateEventContent::RoomTombstone(event);
        old_room.send_state_event(content, "").await?;
    } else {
        println!("Room {} not joined", old_room_id);
    }
    Ok(())
}

pub async fn add_alias(room_id: &str, alias: &str) -> CommandResult {
    let room_id = RoomId::try_from(room_id)
        .with_context(|| format!("Failed to parse '{}' as room ID", room_id))?;
    let alias_id = RoomAliasId::try_from(alias)
        .with_context(|| format!("Failed to parse '{}' as room alias", alias))?;
    let client = restore_session().await?;

    let request = alias::create_alias::Request::new(&alias_id, &room_id);
    client.send(request, None).await?;

    Ok(())
}

pub async fn del_alias(alias: &str) -> CommandResult {
    let alias_id = RoomAliasId::try_from(alias)
        .with_context(|| format!("Failed to parse '{}' as room alias", alias))?;
    let client = restore_session().await?;

    let request = alias::delete_alias::Request::new(&alias_id);
    client
        .send(request, None)
        .await
        .with_context(|| format!("Failed to delete alias '{}'", alias))?;

    Ok(())
}
