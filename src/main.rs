mod configuration;

use clap::Parser;
use configuration::Configuration;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let arguments = Configuration::try_parse()?;
    arguments.validate()?;

    println!("{arguments:?}");
    Ok(())
}
