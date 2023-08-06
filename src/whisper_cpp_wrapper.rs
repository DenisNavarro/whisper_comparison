//! Module adapted from <https://github.com/m1guelpf/whisper-cli-rs/tree/v0.1.4>

mod ffmpeg_decoder;
pub mod model;
mod transcript;
mod utils;
mod whisper;

use anyhow::{bail, ensure, Context};
use camino::Utf8PathBuf;

use model::{Model, Size};
use transcript::Transcript;
use utils::write_to;
use whisper::{Language, Whisper};

pub struct Args {
    /// Name of the Whisper model to use
    pub model: Size,

    /// Language spoken in the audio. Attempts to auto-detect by default.
    pub lang: Option<Language>,

    /// Path to the audio file to transcribe
    pub audio: Utf8PathBuf,

    /// Toggle translation
    pub translate: bool,

    /// Generate timestamps for each word
    pub karaoke: bool,
}

pub fn work(args: Args) -> anyhow::Result<()> {
    let audio = args.audio;
    let file_name = audio.file_name().with_context(|| format!("{audio:?} does not have a name"))?;
    ensure!(audio.exists(), "The provided audio file does not exist.");
    let mut audio_transcripter =
        AudioTranscripter::new(args.model, args.lang, args.translate, args.karaoke)?;
    let transcript = audio_transcripter.transcript(audio.as_ref())?;
    write_to(audio.with_file_name(format!("{file_name}.txt")).as_ref(), &transcript.as_text());
    write_to(audio.with_file_name(format!("{file_name}.vtt")).as_ref(), &transcript.as_vtt());
    write_to(audio.with_file_name(format!("{file_name}.srt")).as_ref(), &transcript.as_srt());
    println!("time: {:?}", transcript.processing_time);
    Ok(())
}

pub struct AudioTranscripter {
    whisper: Whisper,
    translate: bool,
    karaoke: bool,
}

impl AudioTranscripter {
    pub fn new(
        model: Size,
        mut lang: Option<Language>,
        translate: bool,
        karaoke: bool,
    ) -> anyhow::Result<Self> {
        if model.is_english_only() && (lang == Some(Language::Auto) || lang.is_none()) {
            lang = Some(Language::English);
        }
        if model.is_english_only() && lang != Some(Language::English) {
            bail!("The selected model only supports English.");
        }
        let whisper = Whisper::new(&Model::new(model), lang);
        Ok(Self { whisper, translate, karaoke })
    }

    // TODO: replace audio_file_path with a generic reader.
    pub fn transcript(&mut self, audio_file_path: &str) -> anyhow::Result<Transcript> {
        let transcript = self.whisper.transcribe(audio_file_path, self.translate, self.karaoke)?;
        Ok(transcript)
    }

    // TODO: replace audio_file_path with a generic reader.
    pub fn transcript_txt(&mut self, audio_file_path: &str) -> anyhow::Result<String> {
        let transcript = self.transcript(audio_file_path)?;
        Ok(transcript.as_srt())
    }
}
