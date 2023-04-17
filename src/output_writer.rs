use chrono::Duration;
use colored::*;

#[derive(Default)]
pub struct OutputWriter;

impl OutputWriter {
    pub fn write_output(&self, remaining_duration: Duration) {
        let days = remaining_duration.num_days();
        let hours = remaining_duration.num_hours() - days * 24;
        let minutes = remaining_duration.num_minutes() - remaining_duration.num_hours() * 60;

        println!(
            "{}",
            "############################################################".green()
        );
        println!(
            "{:^60}",
            format!("Remaining Time: {days} Days, {hours}:{minutes}h").red()
        );
        println!(
            "{}",
            "############################################################".green()
        );
    }
}
