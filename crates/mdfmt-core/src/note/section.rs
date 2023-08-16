use serde::{Deserialize, Serialize};
use tsify::Tsify;

use super::block::Block;

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
