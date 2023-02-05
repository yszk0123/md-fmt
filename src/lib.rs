mod ast;
pub mod cli;
mod note;

use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use markdown::mdast::Node;
use markdown::{to_mdast, Constructs, ParseOptions};
use note::NotePrinter;

pub use crate::ast::builder;
pub use crate::ast::pretty::pretty;
pub use crate::ast::printer;
use crate::cli::config::Config;
use crate::note::metadata::Metadata;
pub use crate::note::toc;
pub use crate::note::NoteParser;

pub fn to_mdast_from_str(s: &str) -> Result<Node> {
    to_mdast(
        s,
        &ParseOptions {
            constructs: Constructs {
                frontmatter: true,
                ..Constructs::gfm()
            },
            ..ParseOptions::gfm()
        },
    )
    .map_err(|s| anyhow!(s))
}

pub fn format(node: &Node) -> Result<String> {
    let note = NoteParser::parse(node)?;
    let note = note.normalize()?;
    NotePrinter::print(&note)
}

pub fn run(config: &Config) -> Result<()> {
    for file in config.files.iter() {
        run_file(config, file)?;
    }

    Ok(())
}

fn run_file(config: &Config, file: &PathBuf) -> Result<()> {
    let content = fs::read_to_string(file)
        .with_context(|| format!("could not read file `{}`", file.display()))?;

    if config.check {
        let ok = to_mdast_from_str(&content)
            .and_then(|node| format(&node))
            .is_ok();
        println!("{ok}: \"{}\"", file.display());
        return Ok(());
    }

    let node = to_mdast_from_str(&content)
        .with_context(|| format!("could not parse file `{}`", file.display()))?;

    if config.md {
        let s = pretty(&node);
        println!("{s}");
        return Ok(());
    }

    if config.note {
        let note = NoteParser::parse(&node)?;
        let s = note::pretty::pretty(&note);
        println!("{s}");
        return Ok(());
    }

    let content =
        format(&node).with_context(|| format!("could not stringify file `{}`", file.display()))?;

    if config.write {
        fs::write(file, content)
            .with_context(|| format!("could not write file `{}`", file.display()))?;
        return Ok(());
    }

    println!("{content}");

    Ok(())
}
