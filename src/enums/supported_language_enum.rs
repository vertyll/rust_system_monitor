use serde::{Deserialize, Serialize};
use std::fmt;
use unic_langid::{LanguageIdentifier, langid};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SupportedLanguageEnum {
    Polish,
    English,
}

impl SupportedLanguageEnum {
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::Polish, Self::English].iter().copied()
    }

    pub fn id(&self) -> LanguageIdentifier {
        match self {
            Self::Polish => langid!("pl"),
            Self::English => langid!("en-US"),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Polish => "Polski",
            Self::English => "English",
        }
    }
}

impl fmt::Display for SupportedLanguageEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id())
    }
}
