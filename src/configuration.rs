use chrono::{Local, NaiveDate};
use clap::Parser;
use std::error::Error;

#[derive(Parser, Debug)] // requires `derive` feature
pub struct Configuration {
    #[arg(short = 'd', long = "end_date")]
    pub(crate) end_date: NaiveDate,
    #[arg(short = 'e', long = "exclusions", value_parser = parse_exclusions)]
    pub(crate) excluded: Option<Vec<(NaiveDate, NaiveDate)>>,
}

fn parse_exclusions(
    s: &str,
) -> Result<(NaiveDate, NaiveDate), Box<dyn Error + Send + Sync + 'static>> {
    let pos = s
        .find('/')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

impl Configuration {
    pub fn validate(&self) -> Result<(), String> {
        let today = Local::now().date_naive();
        let end_date = &self.end_date;

        let mut messages = vec![];

        if today > *end_date {
            messages.push(format!(
                "Invalid end date provided: {end_date}. It must be in the future"
            ));
        }

        if let Some(exclusions) = &self.excluded {
            for exclusion in exclusions {
                let start = exclusion.0;
                let end = exclusion.1;
                if end < start {
                    messages.push(format!(
                        "Invalid Exclusion provided. End ({end}) needs to be after Start({start})"
                    ))
                }
            }
        }
        if messages.len() > 0 {
            return Err(messages.join("\n"));
        }

        Ok(())
    }
}
