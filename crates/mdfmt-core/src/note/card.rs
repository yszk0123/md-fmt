use serde::{Deserialize, Serialize};
use tsify::Tsify;

use super::{block::Block, note_kind::NoteKind};

#[derive(PartialEq, Debug, Default, Clone, Serialize, Deserialize, Tsify)]
pub struct Card {
    pub kind: NoteKind,
    pub title: Option<String>,
    pub children: Vec<Block>,
}
