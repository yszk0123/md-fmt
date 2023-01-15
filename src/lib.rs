use clap::Parser;
use std::error::Error;
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
    pub fn new() -> Result<Config, &'static str> {
        let args = Args::parse();

        Ok(Config {
            files: args.file.clone(),
            write: args.write,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    for file in config.files {
        let content = fs::read_to_string(&file)?;

        if config.write {
            fs::write(&file, content)?;
        } else {
            println!("{content}");
        }
    }

    Ok(())
}
