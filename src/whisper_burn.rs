//! Code adapted from <https://github.com/Gadersd/whisper-burn/blob/624fefdb0b3961463cc3a08c632c46d4a182d83a/src/main.rs>
//!
//! The 624fefdb0b3961463cc3a08c632c46d4a182d83a commit was on 2023-08-07.

mod audio;
mod helper;
mod model;
mod token;
mod transcribe;

use model::{Whisper, WhisperConfig};
use transcribe::waveform_to_text;

use burn_wgpu::{AutoGraphicsApi, WgpuBackend, WgpuDevice};

use burn::{config::Config, module::Module, tensor::backend::Backend};

use hound::{self, SampleFormat};

#[allow(clippy::cast_precision_loss)]
fn load_audio_waveform(filename: &str) -> hound::Result<(Vec<f32>, usize)> {
    let reader = hound::WavReader::open(filename)?;
    let spec = reader.spec();
    let sample_rate = spec.sample_rate as usize;
    let sample_format = spec.sample_format;
    let max_int_val = 2_u32.pow(u32::from(spec.bits_per_sample) - 1) - 1;
    let floats = match sample_format {
        SampleFormat::Float => reader.into_samples::<f32>().collect::<hound::Result<_>>()?,
        SampleFormat::Int => reader
            .into_samples::<i32>()
            .map(|s| s.map(|s| s as f32 / max_int_val as f32))
            .collect::<hound::Result<_>>()?,
    };
    Ok((floats, sample_rate))
}

use token::Gpt2Tokenizer;

use burn::record::{DefaultRecorder, Recorder, RecorderError};

fn load_whisper_model_file<B: Backend>(
    config: &WhisperConfig,
    filename: &str,
) -> Result<Whisper<B>, RecorderError> {
    DefaultRecorder::new().load(filename.into()).map(|record| config.init().load_record(record))
}

use std::process;

pub fn work(wav_file: &str, model_name: &str) {
    type Backend = WgpuBackend<AutoGraphicsApi, f32, i32>;
    let device = WgpuDevice::BestAvailable;

    println!("Loading waveform...");
    let (waveform, sample_rate) = match load_audio_waveform(wav_file) {
        Ok((w, sr)) => (w, sr),
        Err(e) => {
            eprintln!("Failed to load audio file: {e}");
            process::exit(1);
        }
    };

    let bpe = match Gpt2Tokenizer::new() {
        Ok(bpe) => bpe,
        Err(e) => {
            eprintln!("Failed to load tokenizer: {e}");
            process::exit(1);
        }
    };

    let whisper_config = match WhisperConfig::load(&format!("{model_name}.cfg")) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to load whisper config: {e}");
            process::exit(1);
        }
    };

    println!("Loading model...");
    let whisper: Whisper<Backend> = match load_whisper_model_file(&whisper_config, model_name) {
        Ok(whisper_model) => whisper_model,
        Err(e) => {
            eprintln!("Failed to load whisper model file: {e}");
            process::exit(1);
        }
    };

    let whisper = whisper.to_device(&device);

    let (text, _tokens) = match waveform_to_text(&whisper, &bpe, waveform, sample_rate) {
        Ok((text, tokens)) => (text, tokens),
        Err(e) => {
            eprintln!("Error during transcription: {e}");
            process::exit(1);
        }
    };

    println!("Transcribed text: {text}");
}
