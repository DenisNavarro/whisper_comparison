use std::{fmt::Display, path::PathBuf};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum Size {
    #[clap(name = "tiny.en")]
    TinyEnglish,
    #[clap(name = "tiny")]
    Tiny,
    #[clap(name = "base.en")]
    BaseEnglish,
    #[clap(name = "base")]
    Base,
    #[clap(name = "small.en")]
    SmallEnglish,
    #[clap(name = "small")]
    Small,
    #[clap(name = "medium.en")]
    MediumEnglish,
    #[clap(name = "medium")]
    Medium,
    #[clap(name = "large")]
    Large,
    #[clap(name = "large-v1")]
    LargeV1,
}

impl Size {
    #[must_use]
    pub fn get_path(self) -> PathBuf {
        let mut path = PathBuf::from("whisper_cpp_data");
        path.push(format!("{self}.bin"));
        path
    }

    #[must_use]
    pub const fn is_english_only(self) -> bool {
        matches!(
            self,
            Self::TinyEnglish | Self::BaseEnglish | Self::SmallEnglish | Self::MediumEnglish
        )
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let key = match self {
            Self::TinyEnglish => "tiny.en",
            Self::Tiny => "tiny",
            Self::BaseEnglish => "base.en",
            Self::Base => "base",
            Self::SmallEnglish => "small.en",
            Self::Small => "small",
            Self::MediumEnglish => "medium.en",
            Self::Medium => "medium",
            Self::Large => "large",
            Self::LargeV1 => "large-v1",
        };

        write!(f, "{key}")
    }
}

pub struct Model {
    size: Size,
}

impl Model {
    #[must_use]
    pub const fn new(size: Size) -> Self {
        Self { size }
    }

    #[must_use]
    pub fn get_path(&self) -> PathBuf {
        self.size.get_path()
    }
}
