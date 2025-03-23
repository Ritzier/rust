# WAV to Text

This project uses [whisper-rs](https://github.com/tazz4843/whisper-rs) to perform audio-to-text transcription on WAV
files. [hound](https://github.com/ruuda/hound) is used for reading the WAV files.

## Usage

### 1. Get GGLM Model

- Get GGLM model from [HuggingFace](https://huggingface.co/ggerganov/whisper.cpp/tree/main) and move the models to
  `gglm/`

- You can put mutli models in `gglm`, it would progress for each wav files in `wav/`

### 2. Get a WAV File

- You can find WAV files at various online resources such as [WavSource](https://www.wavsource.com).

### 3. Convert WAV to Target Config

- The `whisper-rs` library and this project require the WAV files to be in a specific format: 16kHz, mono, 16-bit PCM.

- **Verify Audio Format**:

  ```sh

  soxi wav/cant.wav
  # Should show:
  # Channels       : 1
  # Sample Rate    : 16000
  # Precision      : 16-bit
  ```

- Ensure that the output matches the required format. If it doesn't, proceed to the next step.

- **Convert with `ffmpeg`**:

```sh
ffmpeg -i input.wav -ar 16000 -ac 1 -c:a pcm_s16le output.wav
```

- Move them to `wav/`

### 3. Run the Transcription

```sh
cargo run
```
