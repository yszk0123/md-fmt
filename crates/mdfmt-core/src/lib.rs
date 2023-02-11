mod ast;
pub mod cli;
mod note;

use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use glob::glob;
use markdown::mdast::Node;
use markdown::{to_mdast, Constructs, ParseOptions};
use note::NotePrinter;
use once_cell::sync::Lazy;
use regex::{Regex, RegexBuilder};

pub use crate::ast::builder;
pub use crate::ast::pretty::pretty;
pub use crate::ast::printer;
use crate::cli::config::Config;
use crate::note::metadata::Metadata;
pub use crate::note::toc;
pub use crate::note::NoteParser;

static RE: Lazy<Regex> = Lazy::new(|| {
    RegexBuilder::new(r"\[!\[[^]]*\]\([^)]*\)[^]]*\]\([^)]*\)")
        .multi_line(true)
        .build()
        .unwrap()
});

// FIXME: Workaround
// thread 'main' panicked at 'internal error: entered unreachable code: expected footnote refereence, image, or link on stack', /Users/yszk0123/.cargo/registry/src/github.com-1ecc6299db9ec823/markdown-1.0.0-alpha.5/src/to_mdast.rs:1271:14
fn escape(s: &str) -> String {
    RE.replace_all(s, "`$0`").to_string()
}

pub fn to_mdast_from_str(s: &str) -> Result<Node> {
    to_mdast(
        &escape(s),
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

pub fn print_node(node: &Node) -> Result<String> {
    let note = NoteParser::parse(node)?;
    let note = note.normalize()?;
    NotePrinter::print(&note)
}

pub fn run(config: &Config) -> Result<()> {
    if let Some(pattern) = &config.glob {
        for entry in (glob(pattern)?).flatten() {
            if entry.is_file() {
                run_file(config, &entry)?;
            }
        }
    } else {
        for file in config.files.iter() {
            run_file(config, file)?;
        }
    }

    Ok(())
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
