use std::{fs, path::PathBuf};

use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};
use tokio::task;

pub struct StoredDocument {
    pub location: PathBuf,
}

impl StoredDocument {
    pub async fn create<T>(path: PathBuf, data: T) -> Result<Self>
    where
        T: Send + Sync + Serialize + 'static,
    {
        task::spawn_blocking(move || -> Result<StoredDocument> {
            let contents = serde_json::to_string(&data)?;

            fs::write(&path, contents)?;

            Ok(StoredDocument { location: path })
        })
        .await?
    }

    pub async fn read<T>(&self) -> Result<T>
    where
        T: Send + Sync + DeserializeOwned + 'static,
    {
        let path = self.location.clone();

        task::spawn_blocking(|| -> Result<T> {
            let contents = fs::read_to_string(path)?;

            Ok(serde_json::from_str::<T>(&contents)?)
        })
        .await?
    }

    pub async fn write<T>(&self, data: T) -> Result<()>
    where
        T: Send + Sync + Serialize + 'static,
    {
        let path = self.location.clone();

        task::spawn_blocking(move || -> Result<()> {
            let contents = serde_json::to_string(&data)?;

            Ok(fs::write(path, contents)?)
        })
        .await?
    }

    pub async fn delete(self) -> Result<()> {
        task::spawn_blocking(move || Ok(fs::remove_file(self.location)?)).await?
    }
}
