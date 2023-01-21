use anyhow::Result;
use std::env;

use mdfmt::cli::config::Config;

fn main() -> Result<()> {
    let config = Config::build(env::args())?;

    mdfmt::run(config)?;

    Ok(())
}
