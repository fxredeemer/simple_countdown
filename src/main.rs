mod calculator;
mod configuration;
mod output_writer;

use clap::Parser;
use configuration::Configuration;
use output_writer::OutputWriter;

use crate::calculator::Calculator;

fn main() {
    let writer = OutputWriter::default();

    let arguments = Configuration::parse();

    let calculator = Calculator::new(arguments);

    let remaining_duration = calculator.calculate_remaining_duration();

    writer.write_output(&remaining_duration);
}
