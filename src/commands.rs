use matrix_sdk::{identifiers::UserId, Client, Session};
use rpassword::prompt_password_stderr;
use serde_lexpr;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{self, Write};

static SESSION_FILE: &str = "session_info";

pub async fn login(username: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();

    let user_id = match username {
        Some(s) => UserId::try_from(s)?,
        None => {
            let mut s = String::new();

            eprint!("Username: ");
            io::stderr().flush().unwrap();

            stdin.read_line(&mut s)?;
            UserId::try_from(s.trim_end())?
        }
    };

    let password = prompt_password_stderr("Password: ")?;

    let client = Client::new_from_user_id(user_id.clone()).await?;
    println!(
        "{} logged in? {}",
        client.homeserver().await,
        client.logged_in().await
    );

    let response = client
        .login(user_id.localpart(), &password, None, Some("mxadm"))
        .await?;
    println!("{:?}", response);
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
    println!("{:?}", session_info);

    let mut session_file = File::create(SESSION_FILE)?;
    serde_lexpr::to_writer(&mut session_file, &session_info)?;

    Ok(())
}

pub async fn status() -> Result<(), Box<dyn std::error::Error>> {
    let session_file = match File::open(SESSION_FILE) {
        Ok(f) => f,
        _ => {
            println!("Not logged in: no session info file found");
            return Ok(())
        }
    };

    let session_info: Session = serde_lexpr::from_reader(session_file)?;
    let client = Client::new_from_user_id(session_info.user_id.clone()).await?;
    if let Err(e) = client.restore_login(session_info).await {
        println!("Not logged in: unable to restore session; {}", e);
        return Ok(())
    }

    // Need to make a query requiring a valid token to check session validity
    if let Ok(_) = client.devices().await {
        println!(
            "Logged in as {} ({}) with device ID {}",
            client.user_id().await.unwrap(),
            client.display_name().await?.unwrap(),
            client.device_id().await.unwrap()
        );
    } else {
        println!("Not logged in");
    }

    Ok(())
}
