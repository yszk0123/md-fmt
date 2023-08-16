mod ast;
mod chunk;
pub mod cli;
mod index;
mod note;
mod printer;
mod typescript_custom_section;

use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use glob::glob;
use markdown::mdast::Node;
use markdown::{to_mdast, Constructs, ParseOptions};
use note::model::Note;
use once_cell::sync::Lazy;
use regex::{Regex, RegexBuilder};

pub use crate::ast::builder;
pub use crate::ast::pretty::pretty;
use crate::cli::config::Config;
use crate::index::model::Index;
use crate::index::printer::IndexPrinter;
pub use crate::note::metadata;
pub use crate::note::model;
pub use crate::note::toc;
pub use crate::note::NoteParser;
use crate::printer::Printer;

static RE: Lazy<Regex> = Lazy::new(|| {
    RegexBuilder::new(r"\[!\[[^]]*\]\([^)]*\)[^]]*\]\([^)]*\)")
        .multi_line(true)
        .build()
        .unwrap()
});

pub fn format(input: &str) -> Result<String> {
    let node = to_mdast_from_str(input).with_context(|| anyhow!("could not parse file"))?;
    print_node(&node)
}

pub fn parse(input: &str) -> Result<Note> {
    let node = to_mdast_from_str(input).with_context(|| anyhow!("could not parse"))?;
    NoteParser::parse(&node)
}

pub fn stringify(input: &Note) -> Result<String> {
    input.print()
}

// FIXME: Workaround
// thread 'main' panicked at 'internal error: entered unreachable code: expected footnote refereence, image, or link on stack', $HOME/.cargo/registry/src/github.com-1ecc6299db9ec823/markdown-1.0.0-alpha.5/src/to_mdast.rs:1271:14
fn escape(s: &str) -> String {
    RE.replace_all(s, "`$0`").to_string()
}

fn to_mdast_from_str(s: &str) -> Result<Node> {
    to_mdast(
        &escape(s),
        &ParseOptions {
            constructs: Constructs {
                frontmatter: true,
                math_flow: true,
                math_text: true,
                ..Constructs::gfm()
            },
            ..ParseOptions::gfm()
        },
    )
    .map_err(|s| anyhow!(s))
}

fn print_node(node: &Node) -> Result<String> {
    NoteParser::parse(node)?.normalize()?.print()
}

pub fn run(config: &Config) -> Result<()> {
    let entries: Vec<PathBuf> = if let Some(pattern) = &config.glob {
        (glob(pattern)?)
            .flatten()
            .filter(|e| e.is_file())
            .collect::<Vec<PathBuf>>()
    } else {
        config.files.clone()
    };

    if let Some(file) = &config.index {
        let content = generate_index(&entries)?;
        fs::write(file, content).with_context(|| format!("could not write file `{}`", file))?;
        return Ok(());
    }

    for entry in entries {
        run_file(config, &entry)?;
    }

    Ok(())
}

pub fn generate_index(files: &[PathBuf]) -> Result<String> {
    let mut indexes: Vec<Index> = vec![];
    for file in files {
        let content = fs::read_to_string(file)
            .with_context(|| format!("could not read file `{}`", file.display()))?;

        let node = to_mdast_from_str(&content)
            .with_context(|| format!("could not parse file `{}`", file.display()))?;

        let note = NoteParser::parse(&node)?.normalize()?;
        indexes.push(Index::new(file, &note));
    }

    IndexPrinter::print(&indexes)
}

fn run_file(config: &Config, file: &PathBuf) -> Result<()> {
    let content = fs::read_to_string(file)
        .with_context(|| format!("could not read file `{}`", file.display()))?;

    if config.check {
        let err = to_mdast_from_str(&content)
            .and_then(|node| print_node(&node))
            .is_err();
        if err {
            println!("{}", file.display());
        }
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

    if config.json {
        let note = NoteParser::parse(&node)?;
        let s = serde_json::to_string_pretty(&note)?;
        println!("{s}");
        return Ok(());
    }

    let content = print_node(&node)
        .with_context(|| format!("could not stringify file `{}`", file.display()))?;

    if config.write {
        fs::write(file, content)
            .with_context(|| format!("could not write file `{}`", file.display()))?;
        return Ok(());
    }

    println!("{content}");

    Ok(())
}
