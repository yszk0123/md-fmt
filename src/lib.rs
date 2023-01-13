use std::error::Error;
use std::fs;

pub struct Config {
    pub input: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let input = match args.next() {
            Some(arg) => arg,
            None => return Err("required input file path"),
        };
        Ok(Config { input })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.input)?;

    println!("Contents:\n{contents}");

    Ok(())
}
