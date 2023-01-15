use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::PathBuf;

/// Simple Markdown Formatter
#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// Source
    #[arg(short, long)]
    file: Vec<PathBuf>,

    /// Overwrite
    #[arg(short, long, default_value = "false")]
    write: bool,
}

pub struct Config {
    pub files: Vec<PathBuf>,
    pub write: bool,
}

impl Config {
    pub fn build(args: impl Iterator<Item = String>) -> Result<Config> {
        let args =
            Args::try_parse_from(args).with_context(|| format!("could not parse arguments"))?;

        Ok(Config {
            files: args.file.clone(),
            write: args.write,
        })
    }
}

pub fn run(config: Config) -> Result<()> {
    for file in config.files {
        let content = fs::read_to_string(&file)
            .with_context(|| format!("could not read file `{}`", file.display()))?;

        if config.write {
            fs::write(&file, content)
                .with_context(|| format!("could not write file `{}`", file.display()))?;
        } else {
            println!("{content}");
        }
    }

    Ok(())
}
