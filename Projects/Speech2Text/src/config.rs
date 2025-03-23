use std::path::PathBuf;

use tokio::fs;

use crate::*;

// ggml: OpenAI's Whisper models converted to ggml format for use with `whisper.cpp`
// https://huggingface.co/ggerganov/whisper.cpp
// These models are quantized and optimized for efficient inference. They are compatible
// with the `whisper.cpp` library, enabling local, low-resource speech-to-text processing.
// wav: Path to the input WAV audio file that will be transcribed. The audio should be
// in a format compatible with the Whisper model, typically 16kHz mono PCM.
pub struct Config {
    pub ggml: Vec<PathBuf>,
    pub wav: Vec<PathBuf>,
    pub language: String,
}

impl Config {
    pub async fn new() -> Result<Self> {
        let ggml_path = PathBuf::from(format!("{}/ggml/", env!("CARGO_MANIFEST_DIR")));
        let wav_path = PathBuf::from(format!("{}/wav/", env!("CARGO_MANIFEST_DIR")));
        let language = "en".to_string();

        let ggml = get_files(&ggml_path, "bin").await?;
        let wav = get_files(&wav_path, "wav").await?;

        println!("Found {} ggml files", ggml.len());
        println!("Found {} wav files", wav.len());

        Ok(Self {
            ggml,
            wav,
            language,
        })
    }
}

async fn get_files(path: &PathBuf, ext: &str) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    let mut entries = fs::read_dir(&path).await?;
    while let Ok(Some(entry)) = entries.next_entry().await {
        let path = entry.path();
        if let Some(file_ext) = path.extension() {
            if file_ext == ext {
                files.push(path);
            }
        }
    }

    Ok(files)
}
