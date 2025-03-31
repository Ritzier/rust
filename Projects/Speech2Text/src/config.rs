use std::path::PathBuf;

use tokio::fs;

use super::*;

pub struct Config {
    pub gglm: Vec<PathBuf>,
    pub audio: Vec<PathBuf>,
}

impl Config {
    pub async fn new() -> Result<Self> {
        let gglm_path = PathBuf::from(format!("{}/ggml/", env!("CARGO_MANIFEST_DIR")));
        let audio_path = PathBuf::from(format!("{}/audio/", env!("CARGO_MANIFEST_DIR")));

        let gglm = get_files(&gglm_path, vec!["bin"]).await?;
        let audio = get_files(&audio_path, vec!["mp3", "wav"]).await?;

        if gglm.is_empty() {
            return Err(Error::GgmlNotFound);
        }

        if audio.is_empty() {
            return Err(Error::AudioNotFound);
        }

        tracing::info!("Found {} gglm model", gglm.len());
        tracing::info!("Found {} audio", audio.len());

        Ok(Self { gglm, audio })
    }
}

async fn get_files(path: &PathBuf, ext: Vec<&str>) -> Result<Vec<PathBuf>> {
    let mut files = vec![];

    let mut entries = fs::read_dir(&path).await?;
    while let Ok(Some(entry)) = entries.next_entry().await {
        let path = entry.path();
        if ext.contains(
            &path
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or_default(),
        ) {
            files.push(path)
        }
    }

    Ok(files)
}
