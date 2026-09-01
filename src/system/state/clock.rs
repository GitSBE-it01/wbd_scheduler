use time::macros::format_description;
use time::{Date, Duration, OffsetDateTime, UtcOffset};

#[derive(Clone, Debug)]
pub struct AppClock {
    offset: UtcOffset,
}

impl AppClock {
    pub fn new() -> Self {
        let offset = UtcOffset::current_local_offset().unwrap_or_else(|_| UtcOffset::UTC);
        Self { offset }
    }

    pub fn now(&self) -> OffsetDateTime {
        OffsetDateTime::now_utc().to_offset(self.offset)
    }

    pub fn today_date(&self) -> Date {
        self.now().date()
    }

    pub fn today(&self) -> String {
        let format = format_description!("[year]-[month]-[day]");
        self.today_date()
            .format(&format)
            .unwrap_or_else(|_| "1970-01-01".to_string())
    }
    pub fn timestamp(&self) -> String {
        let format = format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]");
        self.now()
            .format(&format)
            .unwrap_or_else(|_| "1970-01-01T00:00:00".to_string())
    }
    pub fn date_stringfy(&self, delta: i64) -> String {
        let format = format_description!("[year]-[month]-[day]");
        let date_stringify = self.today_date() + Duration::days(delta);
        date_stringify
            .format(&format)
            .unwrap_or_else(|_| "1970-01-01".to_string())
    }
    pub fn timestamp_stringfy(&self, delta: i64) -> String {
        let format = format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]");
        let date_stringify = self.today_date() + Duration::days(delta);
        date_stringify
            .format(&format)
            .unwrap_or_else(|_| "1970-01-01T00:00:00".to_string())
    }
}
