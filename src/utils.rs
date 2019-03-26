use chrono::prelude::*;
use time::Duration;

pub fn datetime_from_filetime(nanos_since_windows_epoch: u64) -> DateTime<Utc> {
    DateTime::from_utc(
        NaiveDate::from_ymd(1601, 1, 1).and_hms_nano(0, 0, 0, 0)
            + Duration::microseconds((nanos_since_windows_epoch / 10) as i64),
        Utc,
    )
}
