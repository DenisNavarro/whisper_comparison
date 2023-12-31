//! Code adapted from <https://github.com/m1guelpf/whisper-cli-rs/blob/v0.1.4/src/ffmpeg_decoder.rs>

use anyhow::{anyhow, Result};
use audrey::Reader;
use std::env::temp_dir;
use std::process::Stdio;
use std::{fs::File, process::Command};

// ffmpeg -i input.mp3 -ar 16000 output.wav
fn use_ffmpeg(input_path: &str) -> Result<Vec<i16>> {
    let temp_file = temp_dir().join(format!("{}.wav", uuid::Uuid::new_v4()));
    let mut pid = Command::new("ffmpeg")
        .args([
            "-i",
            input_path,
            "-ar",
            "16000",
            "-ac",
            "1",
            "-c:a",
            "pcm_s16le",
            (temp_file.to_str().unwrap()),
            "-hide_banner",
            "-y",
            "-loglevel",
            "error",
        ])
        .stdin(Stdio::null())
        .spawn()?;

    if pid.wait()?.success() {
        let output = File::open(&temp_file)?;
        let mut reader = Reader::new(output)?;
        let samples: Result<Vec<i16>, _> = reader.samples().collect();
        std::fs::remove_file(temp_file)?;
        samples.map_err(std::convert::Into::into)
    } else {
        Err(anyhow!("unable to convert file"))
    }
}

pub fn read_file(audio_file_path: &str) -> Result<Vec<f32>> {
    let audio_buf = use_ffmpeg(audio_file_path)?;
    Ok(whisper_rs::convert_integer_to_float_audio(&audio_buf))
}
