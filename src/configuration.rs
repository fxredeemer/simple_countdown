use chrono::{Local, NaiveDate};
use clap::Parser;
use std::error::Error;

#[derive(Parser, Debug)] // requires `derive` feature
pub struct Configuration {
    #[arg(short = 'd', long = "end_date", value_parser = parse_target_date)]
    pub(crate) end_date: NaiveDate,
    #[arg(short = 'e', long = "exclusions", value_parser = parse_exclusions)]
    pub(crate) excluded: Option<Vec<(NaiveDate, NaiveDate)>>,
}

fn parse_exclusions(
    exclusions: &str,
) -> Result<(NaiveDate, NaiveDate), Box<dyn Error + Send + Sync + 'static>> {
    let pos = exclusions.find('/').ok_or_else(|| {
        format!("Invalid exclusion date format: Must be yyyy-mm-dd/yyyy-mm-dd in `{exclusions}`")
    })?;

    let begin = exclusions[..pos].parse()?;
    let end = exclusions[pos + 1..].parse()?;

    if end < begin {
        Err(format!(
            "Invalid exclusion provided. End({begin}) needs to be after Start({end})"
        ))?;
    }

    Ok((begin, end))
}

fn parse_target_date(
    exclusions: &str,
) -> Result<NaiveDate, Box<dyn Error + Send + Sync + 'static>> {
    let end_date = exclusions.parse::<NaiveDate>()?;
    let today = Local::now().date_naive();

    if today > end_date {
        Err("Invalid end date provided: {end_date}. It must be in the future")?;
    }

    Ok(end_date)
}