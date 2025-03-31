use std::time::Instant;

use super::*;

pub struct Speech2Text {
    config: Config,
}

impl Speech2Text {
    pub async fn new() -> Result<Self> {
        let config = Config::new().await?;

        whisper::init();

        Ok(Self { config })
    }

    pub async fn run(self) -> Result<()> {
        // Initialize models
        tracing::info!("Initialing models");

        // Create model
        let mut models = Vec::new();
        for gglm in self.config.gglm {
            let model = whisper::create_model(&gglm)?;
            let file_name = gglm
                .file_name()
                .and_then(|f| f.to_str())
                .unwrap_or("Invalid GGML")
                .to_string();
            models.push((model, file_name))
        }

        for audio in &self.config.audio {
            //let sample = audio::run(&audio);
            let audio = audio.to_path_buf();

            // Try to extract file name from path
            // If None, fall back to full path
            // Default to `InvalidAudio` if neither is valid
            let audio_name = audio
                .file_name()
                .and_then(|f| f.to_str())
                .unwrap_or_else(|| audio.to_str().unwrap_or("InvalidAudio"));

            let samples = match audio::run(&audio) {
                Ok(sample) => sample,
                Err(e) => {
                    tracing::error!("Audio {audio_name}: {e}");
                    continue;
                }
            };

            // Process audio with each ggml model
            for (model, ggml_name) in &models {
                let start = Instant::now();
                tracing::info!("Start processing {} with {}", audio_name, ggml_name);

                match whisper::run(model, &samples) {
                    Ok(transcribe) => {
                        let duration = start.elapsed();
                        tracing::info!("Time usage: {}ms", duration.as_millis());

                        for (start, end, text) in transcribe {
                            tracing::info!("[{:5} - {:5}ms] {}", start, end, text);
                        }
                    }
                    Err(e) => {
                        tracing::error!(
                            "Error while process {} with {}: {}",
                            audio_name,
                            ggml_name,
                            e
                        )
                    }
                }
            }
        }

        Ok(())
    }
}
