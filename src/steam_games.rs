use std::path::{Path, PathBuf};

use directories::BaseDirs;

#[derive(Debug, thiserror::Error)]
pub enum AppIdCacheError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
}

#[derive(Debug)]
pub struct AppIdCacheInterface {
    cache_file_path: PathBuf,
}

impl AppIdCacheInterface {
    /// Construct a new `AppIdCacheInterface`
    pub fn new() -> Self {
        // Build the path to the cache file
        let cache_file_dir = BaseDirs::new()
            .map(|dirs| dirs.cache_dir().join("gamels"))
            .unwrap_or_else(|| Path::new("/tmp/gamels").to_path_buf());

        Self {
            cache_file_path: cache_file_dir.join("appid_cache.json"),
        }
    }

    /// Force-invalidate the appid cache file
    pub fn invalidate(&mut self) -> Result<(), AppIdCacheError> {
        // Delete the cache file
        std::fs::remove_file(&self.cache_file_path)?;
        Ok(())
    }

    /// Check if the appid cache is valid
    fn is_valid(&self) -> bool {
        // If the file exists and is newer than 30 minutes, it's valid
        self.cache_file_path.exists()
            && self.cache_file_path.metadata().unwrap().modified().unwrap()
                > std::time::SystemTime::now()
                    .checked_sub(std::time::Duration::from_secs(30 * 60))
                    .unwrap()
    }

    /// Refresh the appid cache with the latest data from the Steam API
    async fn refresh(&mut self) -> Result<(), AppIdCacheError> {
        // Create the cache file directory if it doesn't exist
        if let Some(cache_file_dir) = self.cache_file_path.parent() {
            std::fs::create_dir_all(cache_file_dir)?;
        }

        // Download the appid cache
        let appid_cache = reqwest::get("https://api.steampowered.com/ISteamApps/GetAppList/v2/")
            .await?
            .json::<serde_json::Value>()
            .await?
            .get("applist")
            .unwrap()
            .get("apps")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .map(|app| {
                (
                    app.get("appid").unwrap().as_u64().unwrap(),
                    app.get("name").unwrap().as_str().unwrap().to_string(),
                )
            })
            .collect::<std::collections::HashMap<_, _>>();

        // Write the appid cache to the cache file
        std::fs::write(&self.cache_file_path, serde_json::to_string(&appid_cache)?)?;

        Ok(())
    }

    /// Query the appid cache for the name of an appid
    pub async fn query(&mut self, appid: u64) -> Result<Option<String>, AppIdCacheError> {
        // Make sure the cache is reasonably fresh
        if !self.is_valid() {
            self.refresh().await?;
        }

        // Read the cache file
        let appid_cache = serde_json::from_str::<std::collections::HashMap<u64, String>>(
            &std::fs::read_to_string(&self.cache_file_path)?,
        )?;

        // Return the appid name if it exists
        Ok(appid_cache.get(&appid).cloned())
    }
}
