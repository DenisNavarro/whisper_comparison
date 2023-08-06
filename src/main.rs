#![forbid(unsafe_code)]
//#![warn(clippy::pedantic, clippy::nursery)] // TODO: add this line
#![allow(clippy::all, clippy::nursery, clippy::pedantic)] // TODO: remove this line

use clap::{Parser, Subcommand};

use whisper_comparison::whisper_burn::crash;
use whisper_comparison::whisper_cpp_wrapper::model::Size;
use whisper_comparison::whisper_cpp_wrapper::{work, Args};

#[derive(Parser)]
enum Cli {
    /// Call whisper_burn and crash
    Burn,

    /// Call whisper_cpp_wrapper
    #[command(subcommand)]
    Cpp(Cpp),
}

#[derive(Copy, Clone, PartialEq, Eq, Subcommand, clap::ValueEnum)]
pub enum Cpp {
    #[clap(name = "tiny")]
    Tiny,
    #[clap(name = "base")]
    Base,
    #[clap(name = "small")]
    Small,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli {
        Cli::Burn => call_whisper_burn(),
        Cli::Cpp(model_name) => call_whisper_cpp_wrapper(model_name)?,
    }
    Ok(())
}

fn call_whisper_burn() {
    // Debug:
    // thread 'main' panicked at 'attempt to subtract with overflow', src/whisper_burn/transcribe.rs:110:20
    // Release:
    // thread 'main' panicked at 'slice index starts at 172409 but ends at 168511', src/whisper_burn/transcribe.rs:116:22
    crash("whisper_burn_data/audio.wav", "whisper_burn_data/tiny");
}

fn call_whisper_cpp_wrapper(model_name: Cpp) -> anyhow::Result<()> {
    let model = match model_name {
        Cpp::Tiny => Size::Tiny,
        Cpp::Base => Size::Base,
        Cpp::Small => Size::Small,
    };
    let args = Args {
        model,
        lang: None,
        audio: "whisper_cpp_data/audio.wav".into(),
        translate: false,
        karaoke: false,
    };
    work(args)
}
