use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    // Config
    #[error("Didn't find any GGLM model in `ggml`")]
    GgmlNotFound,
    #[error("Didn't find any audio files in `audio`")]
    AudioNotFound,

    // Audio
    #[error("InvalidAudioFormat")]
    InvalidAudioFormat,
    #[error("Audio: no supported audio tracks")]
    CodecTypeNull,
    #[error("Audio: UnsupportedSampleFormat")]
    UnsupportedSampleFormat,
    #[error("Audio: Unsupport {0}")]
    Unsupoort(String),
    #[error("ResamplerContraction: {0:?}")]
    ResamplerContruction(#[from] rubato::ResamplerConstructionError),
    #[error("Resample: {0:?}")]
    Resample(#[from] rubato::ResampleError),

    // Io
    #[error("Io: {0:?}")]
    Io(#[from] std::io::Error),

    // Whisper (crate)
    #[error("Whisper: {0:?}")]
    Whisper(#[from] whisper_rs::WhisperError),

    // Symphonia (crate)
    #[error("Symphonia: {0:?}")]
    Symphonia(#[from] symphonia::core::errors::Error),
}
