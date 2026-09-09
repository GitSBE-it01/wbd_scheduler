use time::format_description::parse_borrowed;
use time::macros::format_description;
use time::{Date, Duration, OffsetDateTime, PrimitiveDateTime, UtcOffset};

use crate::system::state::error::StateError;

#[derive(Clone, Debug)]
pub struct AppClock {
    offset: UtcOffset,
}
impl Default for AppClock {
    fn default() -> Self {
        Self::new()
    }
}

impl AppClock {
    pub fn new() -> Self {
        let offset = UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);
        Self { offset }
    }

    pub fn now(&self) -> OffsetDateTime {
        OffsetDateTime::now_utc().to_offset(self.offset)
    }

    pub fn today_date(&self) -> Date {
        self.now().date()
    }

    pub fn parse_local_str(&self, date_str: &str) -> Result<OffsetDateTime, StateError> {
        let format = format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]");
        let primitive = PrimitiveDateTime::parse(date_str, &format)?;
        Ok(primitive.assume_offset(self.offset))
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
    pub fn date_stringfy(&self, delta: i64, format_str: Option<&str>) -> String {
        let target_date = self.today_date() + Duration::days(delta);
        let raw_fmt = format_str.unwrap_or("[year]-[month]-[day]");
        parse_borrowed::<1>(raw_fmt)
            .ok()
            .and_then(|items| target_date.format(&items).ok())
            .unwrap_or_else(|| "1970-01-01".to_string())
    }
    pub fn timestamp_stringfy(&self, delta: i64) -> String {
        let format = format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]");
        let date_stringify = self.now() + Duration::seconds(delta);
        date_stringify
            .format(&format)
            .unwrap_or_else(|_| "1970-01-01T00:00:00".to_string())
    }
}
