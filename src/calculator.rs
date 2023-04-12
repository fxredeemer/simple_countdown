use chrono::{Datelike, Duration, Local, NaiveDate, Weekday};

use crate::configuration::Configuration;

pub struct Calculator {
    configuration: Configuration,
    today: NaiveDate,
}

impl Calculator {
    pub fn new(configuration: Configuration) -> Self {
        Self {
            configuration,
            today: Local::now().date_naive(),
        }
    }

    pub fn calculate_remaining_days(&self) -> i32 {
        let total_days = self.configuration.end_date - self.today;

        let day_count = total_days.num_days() as i32;

        let mut work_days = 0;

        for i in 0..day_count {
            let day = self.today + Duration::days(i as i64);

            if self.is_on_weekend(day) {
                continue;
            }

            if self.is_excluded(day) {
                continue;
            }

            work_days += 1;
        }

        work_days
    }

    fn is_on_weekend(&self, day: chrono::NaiveDate) -> bool {
        let week_day = day.weekday();
        week_day == Weekday::Sat || week_day == Weekday::Sun
    }

    fn is_excluded(&self, day: chrono::NaiveDate) -> bool {
        if let Some(exclusions) = &self.configuration.excluded {
            return exclusions.iter().any(|d| day > d.0 && day < d.1);
        }

        false
    }
}
