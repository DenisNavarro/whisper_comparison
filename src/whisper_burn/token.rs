#![allow(clippy::cast_lossless, clippy::cast_possible_truncation, clippy::module_name_repetitions)]

//! Code adapted from <https://github.com/Gadersd/whisper-burn/blob/3757c15fd18fe2ec2c398cb6a4697e108442ff3a/src/token.rs>
//!
//! The 3757c15fd18fe2ec2c398cb6a4697e108442ff3a commit was on 2023-07-30.

use serde::ser::StdError;
use std::result;

use tokenizers::AddedToken;

pub type Result<T> = result::Result<T, Box<(dyn StdError + Send + Sync + 'static)>>;

pub struct Gpt2Tokenizer {
    tokenizer: tokenizers::Tokenizer,
}

impl Gpt2Tokenizer {
    pub fn new() -> Result<Self> {
        let mut tokenizer = tokenizers::Tokenizer::from_pretrained("gpt2", None)?;
        tokenizer.add_special_tokens(&construct_special_tokens());

        Ok(Self { tokenizer })
    }

    pub fn special_token(&self, token: SpecialToken) -> Option<usize> {
        self.tokenizer.token_to_id(&token.to_string()).map(|t| t as usize)
    }

    pub fn decode(&self, tokens: &[usize], skip_special: bool) -> Result<String> {
        self.tokenizer.decode(tokens.iter().map(|t| *t as u32).collect(), skip_special)
    }
}

const LANGUAGES: [&str; 98] = [
    "en", "zh", "de", "es", "ru", "ko", "fr", "ja", "pt", "tr", "pl", "ca", "nl", "ar", "sv", "it",
    "id", "hi", "fi", "vi", "he", "uk", "el", "ms", "cs", "ro", "da", "hu", "ta", "no", "th", "ur",
    "hr", "bg", "lt", "la", "mi", "ml", "cy", "sk", "te", "fa", "lv", "bn", "sr", "az", "sl", "kn",
    "et", "mk", "br", "eu", "is", "hy", "ne", "mn", "bs", "kk", "sq", "sw", "gl", "mr", "pa", "si",
    "km", "sn", "yo", "so", "af", "oc", "ka", "be", "tg", "sd", "gu", "am", "yi", "lo", "uz", "fo",
    "ht", "ps", "tk", "nn", "mt", "sa", "lb", "my", "bo", "tl", "mg", "as", "tt", "ln", "ha", "ba",
    "jw", "su",
];

#[derive(Copy, Clone)]
pub enum SpecialToken {
    EndofText,
    StartofTranscript,
    Transcribe,
    Timestamp(f64),
}

impl ToString for SpecialToken {
    fn to_string(&self) -> String {
        match self {
            Self::EndofText => "<|endoftext|>".into(),
            Self::StartofTranscript => "<|startoftranscript|>".into(),
            Self::Transcribe => "<|transcribe|>".into(),
            Self::Timestamp(val) => format!("<|{val:.2}|>"),
        }
    }
}

fn construct_special_tokens() -> Vec<AddedToken> {
    const SPEC1: [&str; 2] = ["<|endoftext|>", "<|startoftranscript|>"];

    const SPEC2: [&str; 6] = [
        "<|translate|>",
        "<|transcribe|>",
        "<|startoflm|>",
        "<|startofprev|>",
        "<|nospeech|>",
        "<|notimestamps|>",
    ];

    let lang_keys = LANGUAGES.iter().map(|lang| format!("<|{lang}|>"));

    let range_keys = (0..1501).map(|i| i as f64 * 0.02).map(|f| format!("<|{f:.2}|>"));

    SPEC1
        .into_iter()
        .map(String::from)
        .chain(lang_keys.into_iter())
        .chain(SPEC2.into_iter().map(String::from))
        .chain(range_keys.into_iter())
        .map(|tok| AddedToken::from(tok, true))
        .collect()
}
