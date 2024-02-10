use anyhow::Result;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(PartialEq, Debug, Default, Serialize, Deserialize, Clone, Tsify)]
pub enum NoteKind {
    #[default]
    Note,
    Summary,
    Quote,
    Question,
    Toc,
    Todo,
}

impl std::fmt::Display for NoteKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Note => write!(f, "note"),
            Self::Summary => write!(f, "summary"),
            Self::Quote => write!(f, "quote"),
            Self::Question => write!(f, "question"),
            Self::Toc => write!(f, "toc"),
            Self::Todo => write!(f, "todo"),
        }
    }
}

impl std::str::FromStr for NoteKind {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "note" => Ok(Self::Note),
            "summary" => Ok(Self::Summary),
            "quote" => Ok(Self::Quote),
            "question" => Ok(Self::Question),
            "toc" => Ok(Self::Toc),
            "todo" => Ok(Self::Todo),
            _ => Ok(Self::Note),
        }
    }
}
