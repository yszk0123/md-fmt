use std::env;

use anyhow::Result;
use mdfmt::cli::config::Config;

fn main() -> Result<()> {
    let config = Config::build(env::args())?;

    mdfmt::run(config)?;

    Ok(())
}
