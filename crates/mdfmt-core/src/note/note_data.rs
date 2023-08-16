use anyhow::Result;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

use super::block::Block;
use super::metadata::{Meta, Metadata};

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
