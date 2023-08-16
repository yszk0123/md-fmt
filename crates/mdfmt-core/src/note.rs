mod block;
mod builder;
mod card;
mod metadata;
mod note_data;
mod note_kind;
mod parser;
mod pretty;
mod printer;
mod section;
mod toc;

pub use {
    block::*,
    card::*,
    metadata::*,
    note_data::*,
    note_kind::*,
    parser::NoteParser,
    pretty::pretty,
    section::*,
    toc::{FlattenNode, Toc},
};

mod visitor {
    use anyhow::Result;

    use crate::chunk::{Chunk, ChunkPrinter};

    pub struct VisitorContext {
        chunks: ChunkPrinter,
        depth: u8,
    }

    impl VisitorContext {
        pub fn new(depth: u8) -> Self {
            let chunks = ChunkPrinter::new();
            Self { chunks, depth }
        }

        pub fn get_depth(&self) -> u8 {
            self.depth
        }

        pub fn sub(&mut self) -> Self {
            Self::new(self.depth)
        }

        pub fn push(&mut self, chunk: Chunk) {
            self.chunks.push(chunk);
        }

        pub fn dive<F: FnOnce(&mut Self) -> Result<()>>(&mut self, f: F) -> Result<()> {
            self.depth += 1;
            f(self)?;
            self.depth -= 1;
            Ok(())
        }

        pub fn print(&self) -> String {
            self.chunks.print()
        }
    }

    pub trait Visitor {
        fn visit(&self, context: &mut VisitorContext) -> Result<()>;
    }
}
