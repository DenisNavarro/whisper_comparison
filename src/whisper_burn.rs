//! Code adapted from <https://github.com/Gadersd/whisper-burn/blob/3757c15fd18fe2ec2c398cb6a4697e108442ff3a/src/main.rs>
//!
//! The 3757c15fd18fe2ec2c398cb6a4697e108442ff3a commit was on 2023-07-30.

mod audio;
mod helper;
mod model;
mod token;
mod transcribe;

use model::{Whisper, WhisperConfig};
use transcribe::waveform_to_text;

use burn_wgpu::{AutoGraphicsApi, WgpuBackend, WgpuDevice};

use burn::{config::Config, module::Module, tensor::backend::Backend};

use hound;

fn load_audio_waveform(filename: &str) -> hound::Result<(Vec<f32>, usize)> {
    type T = i16;

    let reader = hound::WavReader::open(filename)?;
    let spec = reader.spec();

    let sample_rate = spec.sample_rate as usize;

    let floats = reader
        .into_samples::<T>()
        .map(|s| s.map(|s| f32::from(s) / f32::from(T::MAX)))
        .collect::<hound::Result<_>>()?;

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

pub fn crash(wav_file: &str, model_name: &str) {
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
