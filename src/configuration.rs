use chrono::{Local, NaiveDate};
use clap::Parser;
use std::error::Error;

#[derive(Parser, Debug)] // requires `derive` feature
pub struct Configuration {
    #[arg(short = 'd', long = "end_date", value_parser = parse_target_date)]
    pub end_date: NaiveDate,
    #[arg(short = 'e', long = "exclusions", value_parser = parse_exclusions)]
    pub excluded: Option<Vec<(NaiveDate, NaiveDate)>>,
    #[arg(short = 'o', long = "days_in_office")]
    pub in_office_days: Option<Vec<chrono::Weekday>>,
}

fn parse_exclusions(
    exclusions: &str,
) -> Result<(NaiveDate, NaiveDate), Box<dyn Error + Send + Sync + 'static>> {
    let pos = exclusions.find('/').ok_or_else(|| {
        format!("Invalid exclusion date format: Must be yyyy-mm-dd/yyyy-mm-dd in `{exclusions}`")
    })?;

    let begin = ensure_iso_date_string(&exclusions[..pos]).parse()?;
    let end = ensure_iso_date_string(&exclusions[pos + 1..]).parse()?;

    if end < begin {
        Err(format!(
            "Invalid exclusion provided. End({begin}) needs to be after Start({end})"
        ))?;
    }

    Ok((begin, end))
}

fn parse_target_date(
    target_date: &str,
) -> Result<NaiveDate, Box<dyn Error + Send + Sync + 'static>> {
    let end_date = ensure_iso_date_string(target_date).parse::<NaiveDate>()?;
    let today = Local::now().date_naive();

    if today > end_date {
        Err("Invalid end date provided: {end_date}. It must be in the future")?;
    }

    Ok(end_date)
}

fn ensure_iso_date_string(date: &str) -> String {
    let mut year_month_day_collection: Vec<&str> = date.split('.').collect();

    if year_month_day_collection.len() != 3 {
        return date.to_owned();
    }

    year_month_day_collection.reverse();
    year_month_day_collection.join("-")
}
