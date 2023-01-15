pub mod cli;
mod md;
mod note;

use anyhow::{anyhow, Context, Result};
use markdown::{to_mdast, Constructs, ParseOptions};
use std::fs;

use crate::cli::config::Config;
use crate::md::serializer::to_markdown;
use crate::note::metadata::Metadata;

pub use crate::md::pretty::pretty;
pub use crate::note::{from_note, to_note};

pub fn run(config: Config) -> Result<()> {
    for file in config.files {
        let content = fs::read_to_string(&file)
            .with_context(|| format!("could not read file `{}`", file.display()))?;

        let node = to_mdast(
            &content,
            &ParseOptions {
                constructs: Constructs {
                    frontmatter: true,
                    ..Constructs::gfm()
                },
                ..ParseOptions::gfm()
            },
        )
        .map_err(|s| anyhow!(s))
        .with_context(|| format!("could not parse file `{}`", file.display()))?;

        if config.md {
            let s = pretty(&node);
            println!("{}", s);
            return Ok(());
        }

        if config.note {
            let note = to_note(&node)?;
            println!("{:?}", note);
            return Ok(());
        }

        let content = to_markdown(&node)
            .with_context(|| format!("could not stringify file `{}`", file.display()))?;

        if config.write {
            fs::write(&file, content)
                .with_context(|| format!("could not write file `{}`", file.display()))?;
            return Ok(());
        }

        println!("{content}");
    }

    Ok(())
}
