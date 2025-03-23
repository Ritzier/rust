// source code: https://github.com/tazz4843/whisper-rs/blob/master/examples/basic_use.rs

use std::path::{Path, PathBuf};

use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

use crate::*;

pub fn run(config: Config) -> Result<()> {
    whisper_rs::install_logging_hooks();

    // Initialize models
    let models = config
        .ggml
        .into_iter()
        .map(|path| {
            (
                {
                    WhisperContext::new_with_params(
                        &path.to_string_lossy(),
                        WhisperContextParameters::default(),
                    )
                }
                .unwrap(),
                path,
            )
        })
        .collect::<Vec<_>>();

    // Process each WAV file with all GGML models
    for wav in &config.wav {
        let samples = match load_and_validate_audio(&wav) {
            Ok(sample) => sample,
            Err(e) => {
                eprintln!("Skipping {:?}: {}", wav, e);
                continue;
            }
        };

        for (model, model_path) in &models {
            match transcribe_audio(&samples, &config.language, model) {
                Ok(transcript) => print_transcript(&wav, model_path, transcript),
                Err(e) => eprintln!("Model {} failed: {}", model_path.display(), e),
            }
        }
    }

    Ok(())
}

fn load_and_validate_audio(path: &Path) -> Result<Vec<f32>> {
    let reader = hound::WavReader::open(&path)?;
    let spec = reader.spec();

    // Validate audio format
    if spec.sample_rate != 16_000 || spec.channels != 1 || spec.bits_per_sample != 16 {
        return Err(Error::InvalidAudioFormat);
    }

    // Read samples with proper error handling
    let samples: Vec<i16> = reader.into_samples::<i16>().map(|x| x.unwrap()).collect();

    // Convert to f32 audio (mono)
    let mut float_samples = vec![0.0f32; samples.len()];
    whisper_rs::convert_integer_to_float_audio(&samples, &mut float_samples)?;

    Ok(float_samples)
}

fn transcribe_audio(
    samples: &[f32],
    language: &str,
    ctx: &WhisperContext,
) -> Result<Vec<(i64, i64, String)>> {
    let mut state = ctx.create_state()?;
    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });

    // Configure processing parameters
    params.set_language(Some(language));
    params.set_print_special(false);
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(false);

    // Run inference
    state.full(params, samples)?;

    // Collect results
    let num_segments = state.full_n_segments()?;
    let mut transcript = Vec::with_capacity(num_segments as usize);

    for i in 0..num_segments {
        let segment = state.full_get_segment_text(i)?;
        let start = state.full_get_segment_t0(i)?;
        let end = state.full_get_segment_t1(i)?;
        transcript.push((start, end, segment));
    }

    Ok(transcript)
}

fn print_transcript(wav: &PathBuf, model_path: &Path, transcript: Vec<(i64, i64, String)>) {
    println!(
        "\nTranscription for {:?} using {:?}",
        wav.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown WAV"),
        model_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown Model"),
    );
    for (start, end, text) in transcript {
        println!("[{:5} - {:5}ms] {}", start, end, text);
    }
}
