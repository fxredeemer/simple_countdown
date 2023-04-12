fn main() {
    let arguments = CliArguments::parse();

    println!("{arguments:?}")
}


use std::error::Error;

use clap::Parser;
use chrono::NaiveDate;

#[derive(Parser, Debug)] // requires `derive` feature
struct CliArguments {
    #[arg(short = 'd', long = "end_date")]
    end_date: NaiveDate,
    #[arg(short = 'e', long = "exclusions", value_parser = parse_exclusions)]
    excluded: Option<Vec<(NaiveDate, NaiveDate)>>,
}


fn parse_exclusions(s: &str) -> Result<(NaiveDate, NaiveDate), Box<dyn Error + Send + Sync + 'static>>
{
    let pos = s
        .find('/')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}