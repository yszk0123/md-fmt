use anyhow::Result;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::metadata::{Meta, Metadata};
use crate::toc::FlattenNode;

#[derive(PartialEq, Debug, Default, Serialize, Deserialize, Tsify)]
pub struct Note {
    pub metadata: Option<Metadata>,
    pub body: Vec<Block>,
}

impl Note {
    pub fn new(metadata: Option<Metadata>, body: Vec<Block>) -> Self {
        Self { metadata, body }
    }

    pub fn normalize(self) -> Result<Self> {
        let mut body = self.get_toc()?;
        body.extend(self.body);

        Ok(Self {
            metadata: self.metadata.and_then(Metadata::normalize),
            body,
        })
    }

    fn get_toc(&self) -> Result<Vec<Block>> {
        match &self.metadata {
            Some(Metadata::Meta(Meta {
                bookmark: Some(b), ..
            })) => {
                if let Some(toc) = b.parse_toc()? {
                    Ok(vec![Block::toc(toc.flatten_ref())])
                } else {
                    Ok(vec![])
                }
            },
            Some(Metadata::Meta(m)) => {
                if let Some(toc) = m.parse_toc()? {
                    Ok(vec![Block::toc(toc.flatten_ref())])
                } else {
                    Ok(vec![])
                }
            },
            _ => Ok(vec![]),
        }
    }
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize, Tsify)]
#[serde(tag = "type", content = "value")]
pub enum Block {
    #[default]
    Empty,
    AnonymousSection(Vec<Block>),
    Section(Section),
    Card(Card),
    Text(String),
    Single(String),
    Toc(Vec<FlattenNode>),
}

impl Block {
    pub fn anonymous_section(children: Vec<Block>) -> Self {
        Self::AnonymousSection(children)
    }

    pub fn section(title: &str, children: Vec<Block>) -> Self {
        Self::Section(Section {
            title: title.to_string(),
            children,
        })
    }

    pub fn toc(children: Vec<FlattenNode>) -> Self {
        Self::Toc(children)
    }

    pub fn card(kind: NoteKind, title: Option<String>, children: Vec<Block>) -> Self {
        Self::Card(Card {
            kind,
            title,
            children,
        })
    }

    pub fn single(text: &str) -> Self {
        Self::Single(text.to_string())
    }

    pub fn text(text: &str) -> Self {
        Self::Text(text.to_string())
    }
}

#[derive(PartialEq, Debug, Default, Clone, Serialize, Deserialize, Tsify)]
pub struct Section {
    pub title: String,
    pub children: Vec<Block>,
}

impl Section {
    pub fn new(title: &str, children: Vec<Block>) -> Self {
        Self {
            title: title.to_string(),
            children,
        }
    }
}

#[derive(PartialEq, Debug, Default, Clone, Serialize, Deserialize, Tsify)]
pub struct Card {
    pub kind: NoteKind,
    pub title: Option<String>,
    pub children: Vec<Block>,
}

#[derive(PartialEq, Debug, Default, Serialize, Deserialize, Clone, Tsify)]
pub enum NoteKind {
    #[default]
    Note,
    Summary,
    Quote,
    Question,
    Toc,
}

impl std::fmt::Display for NoteKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Note => write!(f, "note"),
            Self::Summary => write!(f, "summary"),
            Self::Quote => write!(f, "quote"),
            Self::Question => write!(f, "question"),
            Self::Toc => write!(f, "toc"),
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
            _ => Ok(Self::Note),
        }
    }
}
