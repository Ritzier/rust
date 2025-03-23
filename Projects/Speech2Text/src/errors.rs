use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(
        "GGML not found, get it from https://huggingface.co/ggerganov/whisper.cpp/blob/main/ggml-tiny.bin"
    )]
    GgmlNotFound,
    #[error("Wav not found")]
    WavNotFound,
    #[error("Whisper: {0:?}")]
    Whisper(#[from] whisper_rs::WhisperError),
    #[error("Hound: {0:?}")]
    Hound(#[from] hound::Error),
    #[error("Invalid audio format - must be 16kHz mono 16-bit PCM")]
    InvalidAudioFormat,
    #[error("Io: {0:?}")]
    Io(#[from] std::io::Error),
}
