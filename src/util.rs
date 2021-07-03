use matrix_sdk::{Client, ClientConfig, Session};
use serde_lexpr;
use std::fs::File;

static SESSION_FILE: &str = "session_info";

pub fn build_client_config() -> ClientConfig {
    ClientConfig::new().store_path("./store")
}

pub fn save_session(session: Session) -> Result<(), Box<dyn std::error::Error>> {
    let mut session_file = File::create(SESSION_FILE)?;
    serde_lexpr::to_writer(&mut session_file, &session)?;
    Ok(())
}

pub async fn restore_session() -> Result<Client, Box<dyn std::error::Error>> {
    let session_file = File::open(SESSION_FILE)?;
    let session_info: Session = serde_lexpr::from_reader(session_file)?;

    let client =
        Client::new_from_user_id_with_config(session_info.user_id.clone(), build_client_config())
            .await?;
    client.restore_login(session_info).await?;
    Ok(client)
}
