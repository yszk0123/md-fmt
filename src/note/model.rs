use markdown::mdast::Node;

use crate::Metadata;

#[derive(PartialEq, Debug, Default)]
pub struct Note {
    pub metadata: Option<Metadata>,
    pub head: Block,
    pub body: Block,
}

impl Note {
    pub fn new(metadata: Option<Metadata>, head: Block, body: Block) -> Self {
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
pub enum NoteNode {
    Empty,
    Block(Block),
    Card(Card),
    Node(Node),
}

impl NoteNode {
    pub fn card(kind: NoteKind, children: Vec<NoteNode>) -> Self {
        Self::Card(Card { kind, children })
    }

    pub fn block(title: impl ToString, children: Vec<NoteNode>) -> Self {
        Self::Block(Block {
            title: Some(title.to_string()),
            children,
        })
    }

    pub fn node(node: Node) -> Self {
        Self::Node(node)
    }
}

#[derive(PartialEq, Debug, Default)]
pub struct Block {
    pub title: Option<String>,
    pub children: Vec<NoteNode>,
}

impl Block {
    pub fn new(title: Option<String>, children: Vec<NoteNode>) -> Self {
        Self { title, children }
    }

    pub fn children(children: Vec<NoteNode>) -> Self {
        Self {
            title: None,
            children,
        }
    }
}

#[derive(PartialEq, Debug, Default)]
pub struct Card {
    pub kind: NoteKind,
    pub children: Vec<NoteNode>,
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
