use chrono::{DateTime, Datelike, Duration, Local, NaiveTime, Timelike, Weekday};

use crate::configuration::Configuration;

pub struct Calculator {
    configuration: Configuration,
    now: DateTime<Local>,
}

impl Calculator {
    pub fn new(configuration: Configuration) -> Self {
        Self {
            configuration,
            now: Local::now(),
        }
    }

    pub fn calculate_remaining_duration(&self) -> Duration {
        let work_days = self.calculate_remaining_full_days();

        let addition = match self.now.time().hour() {
            0..=12 => 30,
            _ => 0,
        };

        let end_of_day = NaiveTime::from_hms_opt(17, 30, 00).unwrap();

        let remaining_time_today = end_of_day - self.now.time() - Duration::minutes(addition);

        Duration::days(work_days) + remaining_time_today
    }

    fn calculate_remaining_full_days(&self) -> i64 {
        let today = self.now.date_naive();
        let total_days = self.configuration.end_date - today;

        let day_count = total_days.num_days() as i32;

        let mut work_days = 0;

        for i in 0..day_count {
            let day = today + Duration::days(i as i64);

            if self.is_on_weekend(day) {
                continue;
            }

            if self.is_excluded(day) {
                continue;
            }

            work_days += 1;
        }
        work_days - 1
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
