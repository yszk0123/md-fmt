use serde::{Deserialize, Serialize};
use tsify::Tsify;

use super::card::Card;
use super::note_kind::NoteKind;
use super::section::Section;
use super::toc::FlattenNode;

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
