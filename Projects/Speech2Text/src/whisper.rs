use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

use super::*;
use std::path::PathBuf;

pub fn init() {
    whisper_rs::install_logging_hooks();
}

pub fn create_model(path: &PathBuf) -> Result<WhisperContext> {
    let model = WhisperContext::new_with_params(
        &path.to_string_lossy(),
        WhisperContextParameters::default(),
    )?;

    Ok(model)
}

pub fn run(model: &WhisperContext, samples: &[f32]) -> Result<Vec<(i64, i64, String)>> {
    // Start transcribe audio
    let transcribe = transcribe_audio(&samples, &model)?;

    Ok(transcribe)
}

fn transcribe_audio(samples: &[f32], ctx: &WhisperContext) -> Result<Vec<(i64, i64, String)>> {
    let mut state = ctx.create_state()?;
    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });

    // Configure processing parameters
    params.set_language(Some("en"));
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
