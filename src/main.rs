mod calculator;
mod configuration;

use clap::Parser;
use configuration::Configuration;

use crate::calculator::Calculator;

fn main() {
    let arguments = Configuration::parse();

    println!("{arguments:?}");

    let calculator = Calculator::new(arguments);

    let days_remaining = calculator.calculate_remaining_days();

    println!("remaining days: {days_remaining}");
}
