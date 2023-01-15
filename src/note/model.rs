use markdown::mdast::Node;

use crate::Metadata;

#[derive(Debug)]
pub struct Note {
    pub metadata: Option<Metadata>,
    pub summary: Option<Block>,
    pub blocks: Vec<Block>,
}

#[derive(Debug)]
pub enum Block {
    Empty,
    Container {
        children: Vec<Block>,
    },
    Card {
        kind: CardKind,
        block: Option<Box<Block>>,
    },
    Node(Node),
}

#[derive(Debug)]
pub enum CardKind {
    Note,
    Summary,
    Quote,
    Question,
}
