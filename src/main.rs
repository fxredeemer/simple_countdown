mod configuration;

use clap::Parser;
use configuration::Configuration;

fn main() {
    let arguments = Configuration::parse();

    println!("{arguments:?}");
}
