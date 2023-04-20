use chrono::Duration;
use colored::*;

#[derive(Default)]
pub struct OutputWriter;

impl OutputWriter {
    pub fn write_output(&self, remaining_duration: &Duration) {
        println!(
            "{}",
            "############################################################".green()
        );
        println!("{:^60}", format_duration(&remaining_duration).red());
        println!(
            "{}",
            "############################################################".green()
        );
    }
}

fn format_duration(duration: &Duration) -> String {
    let days = duration.num_days();
    let hours = duration.num_hours() - days * 24;
    let minutes = duration.num_minutes() - duration.num_hours() * 60;

    format!("Remaining Time: {days} Days, {hours}:{minutes:02}h")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let duration = Duration::days(33) + Duration::hours(7) + Duration::minutes(7);

        assert_eq!(format_duration(&duration), "Remaining Time: 33 Days, 7:07h");
    }
}
