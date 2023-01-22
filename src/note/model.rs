use markdown::mdast::Node;

use crate::Metadata;

#[derive(PartialEq, Debug)]
pub struct Note {
    pub metadata: Option<Metadata>,
    pub summary: Option<Block>,
    pub blocks: Vec<Block>,
}

#[derive(PartialEq, Debug)]
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

#[derive(PartialEq, Debug)]
pub enum CardKind {
    Note,
    Summary,
    Quote,
    Question,
}
