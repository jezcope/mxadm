use directories::ProjectDirs;
use matrix_sdk::{Client, ClientConfig, Session};
use serde_lexpr;
use std::fs::{create_dir_all, File};

lazy_static! {
    static ref PROJECT_DIRS: ProjectDirs = ProjectDirs::from("me", "petrichor", "mxadm")
        .expect("Project directories should be available on all platforms");
}
static SESSION_FILE: &str = "session_info";

pub fn build_client_config() -> ClientConfig {
    ClientConfig::new().store_path(PROJECT_DIRS.cache_dir().join("store"))
}

pub fn save_session(session: Session) -> Result<(), Box<dyn std::error::Error>> {
    let cache_dir = PROJECT_DIRS.cache_dir();
    if !cache_dir.exists() {
        create_dir_all(cache_dir)?;
    }
    let mut session_file = File::create(cache_dir.join(SESSION_FILE))?;
    serde_lexpr::to_writer(&mut session_file, &session)?;
    Ok(())
}

pub async fn restore_session() -> Result<Client, Box<dyn std::error::Error>> {
    let session_file = File::open(PROJECT_DIRS.cache_dir().join(SESSION_FILE))?;
    let session_info: Session = serde_lexpr::from_reader(session_file)?;

    let client =
        Client::new_from_user_id_with_config(session_info.user_id.clone(), build_client_config())
            .await?;
    client.restore_login(session_info).await?;
    Ok(client)
}
