use markdown::mdast::Node;

use crate::Metadata;

#[derive(PartialEq, Debug, Default)]
pub struct Note {
    pub metadata: Option<Metadata>,
    pub head: Section,
    pub body: Section,
}

impl Note {
    pub fn new(metadata: Option<Metadata>, head: Section, body: Section) -> Self {
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

#[derive(PartialEq, Debug)]
pub enum Block {
    Empty,
    Section(Section),
    Card(Card),
    Node(Node),
}

impl Block {
    pub fn card(kind: NoteKind, children: Vec<Block>) -> Self {
        Self::Card(Card { kind, children })
    }

    pub fn section(title: impl ToString, children: Vec<Block>) -> Self {
        Self::Section(Section {
            title: Some(title.to_string()),
            children,
        })
    }

    pub fn node(node: Node) -> Self {
        Self::Node(node)
    }
}

#[derive(PartialEq, Debug, Default)]
pub struct Section {
    pub title: Option<String>,
    pub children: Vec<Block>,
}

impl Section {
    pub fn new(title: Option<String>, children: Vec<Block>) -> Self {
        Self { title, children }
    }

    pub fn children(children: Vec<Block>) -> Self {
        Self {
            title: None,
            children,
        }
    }
}

#[derive(PartialEq, Debug, Default)]
pub struct Card {
    pub kind: NoteKind,
    pub children: Vec<Block>,
}

#[derive(PartialEq, Debug, Default)]
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
