use chrono::{DateTime, Datelike, Duration, Local, NaiveTime, Timelike, Weekday};

use crate::configuration::Configuration;

pub struct RemainingDuration {
    pub total_duration: Duration,
    pub in_office_day_count: Option<i32>,
}

struct RemainingDays {
    full_days: i64,
    in_office_day_count: Option<i32>,
}

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

    pub fn calculate_remaining_duration(&self) -> RemainingDuration {
        let remaining_days = self.calculate_remaining_days();

        let addition = match self.now.time().hour() {
            0..=11 => 30,
            _ => 0,
        };

        let full_work_days = remaining_days.full_days;
        let end_of_day = NaiveTime::from_hms_opt(17, 30, 00).unwrap();

        let remaining_time_today = end_of_day - self.now.time() - Duration::minutes(addition);

        let total_duration = Duration::days(full_work_days) + remaining_time_today;

        RemainingDuration {
            total_duration,
            in_office_day_count: remaining_days.in_office_day_count,
        }
    }

    fn calculate_remaining_days(&self) -> RemainingDays {
        let today = self.now.date_naive();
        let total_days = self.configuration.end_date - today;
        let mut in_office_day_count = None;

        let day_count = total_days.num_days() as i32;

        let mut full_days = 0;

        for i in 0..day_count {
            let day = today + Duration::days(i as i64);

            if self.is_on_weekend(day) {
                continue;
            }

            if self.is_excluded(day) {
                continue;
            }

            self.count_in_office_days(day, &mut in_office_day_count);

            full_days += 1;
        }

        RemainingDays {
            full_days,
            in_office_day_count,
        }
    }

    fn count_in_office_days(&self, day: chrono::NaiveDate, in_office_day_count: &mut Option<i32>) {
        if let Some(in_office_days) = &self.configuration.in_office_days {
            if in_office_days.contains(&day.weekday()) {
                *in_office_day_count = match *in_office_day_count {
                    None => Some(1),
                    Some(i) => Some(i + 1),
                }
            }
        }
    }

    fn is_on_weekend(&self, day: chrono::NaiveDate) -> bool {
        let week_day = day.weekday();
        week_day == Weekday::Sat || week_day == Weekday::Sun
    }

    fn is_excluded(&self, day: chrono::NaiveDate) -> bool {
        if let Some(exclusions) = &self.configuration.excluded {
            return exclusions.iter().any(|d| day >= d.0 && day <= d.1);
        }

        false
    }
}
