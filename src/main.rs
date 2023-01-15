use anyhow::Result;
use mdfmt::Config;
use std::env;

fn main() -> Result<()> {
    let config = Config::build(env::args())?;

    mdfmt::run(config)?;

    Ok(())
}
