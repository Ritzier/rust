use rubato::VecResampler;
use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::codecs::{CODEC_TYPE_NULL, DecoderOptions};
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use super::*;
use std::fs::File;
use std::path::PathBuf;

/// Read audio to Vec<i16>
/// # Notes
/// - if mp3, convert to wav
/// - if wav, validate it
pub fn run(path: &PathBuf) -> Result<Vec<f32>> {
    let audio_extension = path
        .extension()
        .and_then(|f| f.to_str())
        .unwrap_or_default();
    match audio_extension {
        "mp3" => transcode(&path, None),
        "wav" => {
            let mut hint = Hint::default();
            hint.with_extension("wav");
            transcode(&path, Some(&hint))
        }
        ext => Err(Error::Unsupoort(ext.to_string())),
    }
}

fn transcode(input: &PathBuf, hint: Option<&Hint>) -> Result<Vec<f32>> {
    // Open audio
    let src = File::open(&input)?;

    // Create media source stream
    let mss = MediaSourceStream::new(Box::new(src), Default::default());

    // Hint
    let hint = match hint {
        Some(hint) => hint,
        None => &Hint::default(),
    };

    // Probe media source
    let probed = symphonia::default::get_probe().format(
        hint,
        mss,
        &FormatOptions::default(),
        &MetadataOptions::default(),
    )?;

    // Get the instantiated format reader
    let mut format = probed.format;

    // Find the first audio track with a known (decodeable) codec
    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .ok_or(Error::CodecTypeNull)?;

    // Create a decoder for the track
    let mut decoder =
        symphonia::default::get_codecs().make(&track.codec_params, &DecoderOptions::default())?;

    // Store track identifier
    let track_id = track.id;

    // Get the original sample rate
    let original_sample_rate = track.codec_params.sample_rate.unwrap_or(44100);
    let target_sample_rate = 16000;

    // Collect all samples first
    let mut original_samples = Vec::new();

    while let Ok(packet) = format.next_packet() {
        if packet.track_id() == track_id {
            if let Ok(decoded) = decoder.decode(&packet) {
                match decoded {
                    AudioBufferRef::U8(buf) => {
                        original_samples.extend(
                            buf.chan(0)
                                .iter()
                                .map(|&s| ((s as i16 - 128) * 256) as f32 / i16::MAX as f32),
                        );
                    }
                    AudioBufferRef::U16(_buf) => {
                        return Err(Error::UnsupportedSampleFormat);
                    }
                    AudioBufferRef::U24(_buf) => {
                        return Err(Error::UnsupportedSampleFormat);
                    }
                    AudioBufferRef::U32(buf) => {
                        original_samples.extend(
                            buf.chan(0)
                                .iter()
                                .map(|&s| ((s >> 16) as i16) as f32 / i16::MAX as f32),
                        );
                    }
                    AudioBufferRef::S8(buf) => {
                        original_samples.extend(
                            buf.chan(0)
                                .iter()
                                .map(|&s| (s as i16 * 256) as f32 / i16::MAX as f32),
                        );
                    }
                    AudioBufferRef::S16(buf) => {
                        original_samples
                            .extend(buf.chan(0).iter().map(|&s| s as f32 / i16::MAX as f32));
                    }
                    AudioBufferRef::S24(_buf) => {
                        return Err(Error::UnsupportedSampleFormat);
                    }
                    AudioBufferRef::S32(buf) => {
                        original_samples.extend(
                            buf.chan(0)
                                .iter()
                                .map(|&s| ((s >> 16) as i16) as f32 / i16::MAX as f32),
                        );
                    }
                    AudioBufferRef::F32(buf) => {
                        original_samples.extend(buf.chan(0).iter().copied());
                    }
                    AudioBufferRef::F64(buf) => {
                        original_samples.extend(buf.chan(0).iter().map(|&s| s as f32));
                    }
                }
            }
        }
    }

    // Set up the resampler
    let params = rubato::SincInterpolationParameters {
        sinc_len: 256,
        f_cutoff: 0.95,
        interpolation: rubato::SincInterpolationType::Linear,
        oversampling_factor: 256,
        window: rubato::WindowFunction::BlackmanHarris2,
    };

    // Create a resampler with fixed input size
    let mut resampler = rubato::SincFixedIn::<f32>::new(
        target_sample_rate as f64 / original_sample_rate as f64,
        2.0,
        params,
        1024, // chunk size
        1,    // channels
    )?;

    // Prepare input in the format expected by rubato (Vec<Vec<f64>>)
    let chunk_size = resampler.input_frames_next();
    let mut input_buffer = vec![Vec::new()];

    // Process the audio in chunks
    let mut output_samples = Vec::new();

    for chunk in original_samples.chunks(chunk_size) {
        // Clear and refill the input buffer
        input_buffer[0].clear();
        input_buffer[0].extend_from_slice(chunk);

        // If the last chunk is smaller than chunk_size, pad with zeros
        if chunk.len() < chunk_size {
            input_buffer[0].resize(chunk_size, 0.0);
        }

        // Process the chunk
        let output = resampler.process(&input_buffer, None)?;

        // Collect the output
        output_samples.extend(output[0].iter());
    }

    // Process any remaining frames
    let output = resampler.process_partial(None, None)?;

    if !output.is_empty() && !output[0].is_empty() {
        output_samples.extend(output[0].iter());
    }

    Ok(output_samples)
}
