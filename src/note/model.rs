use serde::{Deserialize, Serialize};

use crate::Metadata;

#[derive(PartialEq, Debug, Default)]
pub struct Note {
    pub metadata: Option<Metadata>,
    pub head: Vec<Block>,
    pub body: Vec<Section>,
}

impl Note {
    pub fn new(metadata: Option<Metadata>, head: Vec<Block>, body: Vec<Section>) -> Self {
        Self {
            metadata,
            head,
            body,
        }
    }

    pub fn normalize(self) -> Self {
        Self {
            metadata: self.metadata.map(Metadata::normalize),
            head: self.head,
            body: self.body,
        }
    }
}

#[derive(PartialEq, Default, Debug)]
pub enum Block {
    #[default]
    Empty,
    Section(Section),
    Card(Card),
    Text(String),
}

impl Block {
    pub fn section(title: impl ToString, children: Vec<Block>) -> Self {
        Self::Section(Section {
            title: title.to_string(),
            children,
        })
    }

    pub fn card(kind: NoteKind, children: Vec<Block>) -> Self {
        Self::Card(Card { kind, children })
    }

    pub fn text(text: impl ToString) -> Self {
        Self::Text(text.to_string())
    }
}

#[derive(PartialEq, Debug, Default)]
pub struct Section {
    pub title: String,
    pub children: Vec<Block>,
}

impl Section {
    pub fn new(title: impl ToString, children: Vec<Block>) -> Self {
        Self {
            title: title.to_string(),
            children,
        }
    }
}

#[derive(PartialEq, Debug, Default)]
pub struct Card {
    pub kind: NoteKind,
    pub children: Vec<Block>,
}

#[derive(PartialEq, Debug, Default, Serialize, Deserialize)]
pub enum NoteKind {
    #[default]
    Note,
    Summary,
    Quote,
    Question,
}

impl std::fmt::Display for NoteKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Note => write!(f, "note"),
            Self::Summary => write!(f, "summary"),
            Self::Quote => write!(f, "quote"),
            Self::Question => write!(f, "question"),
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
            _ => Ok(Self::Note),
        }
    }
}
