use colored::*;

#[derive(Default)]
pub struct OutputWriter;

impl OutputWriter {
    pub fn write_output(&self, days_remaining: i32) {
        println!(
            "{}",
            "#########################################################".green()
        );
        println!("{:^50}", format!("Remaining days: {days_remaining}").red());
        println!(
            "{}",
            "#########################################################".green()
        );
    }
}
