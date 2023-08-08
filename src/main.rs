#![forbid(unsafe_code)]
#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::missing_errors_doc)]

use clap::{Parser, Subcommand};

use whisper_comparison::whisper_burn;
use whisper_comparison::whisper_cpp_wrapper::model::Size;
use whisper_comparison::whisper_cpp_wrapper::{self, Args};

#[derive(Parser)]
enum Cli {
    /// Call whisper_burn, but the output is not what is expected
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
    // Transcribed text: ~~
    whisper_burn::work("whisper_burn_data/audio.wav", "whisper_burn_data/tiny");
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
    whisper_cpp_wrapper::work(args)
}
