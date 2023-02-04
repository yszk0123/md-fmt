use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    toc::{FlattenNode, Toc},
    Metadata,
};

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

    pub fn normalize(self) -> Result<Self> {
        let mut head = self.get_toc()?;
        head.extend(self.head);

        Ok(Self {
            metadata: self.metadata.and_then(Metadata::normalize),
            head,
            body: self.body,
        })
    }

    fn get_toc(&self) -> Result<Vec<Block>> {
        if let Some(Metadata {
            bookmark: Some(b), ..
        }) = &self.metadata
        {
            if let Some(Toc(nodes)) = b.parse_toc()? {
                let nodes: Vec<FlattenNode> = nodes.iter().flat_map(|v| v.flatten_ref()).collect();
                Ok(vec![Block::toc(nodes)])
            } else {
                Ok(vec![])
            }
        } else {
            Ok(vec![])
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
    Toc(Vec<FlattenNode>),
}

impl Block {
    pub fn section(title: impl ToString, children: Vec<Block>) -> Self {
        Self::Section(Section {
            title: title.to_string(),
            children,
        })
    }

    pub fn toc(children: Vec<FlattenNode>) -> Self {
        Self::Toc(children)
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
