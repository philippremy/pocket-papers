use std::{path::PathBuf, time::Duration};
use log::warn;
use tokio::sync::OnceCell;

use crate::trait_helpers::AppError;

pub static SERVER_DIR: OnceCell<PathBuf> = OnceCell::const_new();

pub async fn create_uuid_dir(uuid: &str) -> Result<PathBuf, AppError> {
    
    // Initialize the server dir
    let server_dir = SERVER_DIR.get_or_try_init::<anyhow::Error, _, _>(async || {
        let data_dir = directories::ProjectDirs::from("de", "philippremy", "backend-server").unwrap().data_dir().to_path_buf();

        if std::fs::exists(&data_dir)? {
            return Ok(data_dir)
        }

        std::fs::create_dir(&data_dir)?;

        Ok(data_dir)
    }).await?;

    std::fs::create_dir(server_dir.join(uuid))?;

    Ok(server_dir.join(uuid))
}

pub async fn get_uuid_dir(uuid: &str) -> Result<Option<PathBuf>, AppError> {

    // Initialize the server dir
    let server_dir = SERVER_DIR.get_or_try_init::<anyhow::Error, _, _>(async || {
        let data_dir = directories::ProjectDirs::from("de", "philippremy", "backend-server").unwrap().data_dir().to_path_buf();
    
        if std::fs::exists(&data_dir)? {
            return Ok(data_dir)
        }
    
        std::fs::create_dir(&data_dir)?;
    
        Ok(data_dir)
    }).await?;

    if !std::fs::exists(server_dir.join(uuid))? {
        return Ok(None)
    }
    
    Ok(Some(server_dir.join(uuid)))

}

pub async fn unpack_files_to_dir(folder: &PathBuf, name: &str, contents: &[u8]) -> Result<(), AppError> {

    std::fs::write(folder.join(name), contents)?;
    Ok(())

}

pub async fn defer_folder_deletion(uuid_borrowed: &str) {

    let uuid = uuid_borrowed.to_string();

    let _ = tokio::spawn(async move {

        tokio::time::sleep(Duration::from_secs(60 * 60)).await;
        if let Ok(path_opt) = get_uuid_dir(&uuid).await {
            if let Some(path) = path_opt {
                match std::fs::remove_dir_all(path) {
                    Ok(_) => {},
                    Err(err) => {
                        warn!("User folder for UUID: {} could not be deleted. Resources may be leaked. Error: {}", uuid, err);
                    }
                }
            }
        }

    });

}