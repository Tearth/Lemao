use std::time::SystemTime;
use std::time::UNIX_EPOCH;

pub struct DateTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub millisecond: u32,
}

impl DateTime {
    pub fn new(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8, millisecond: u32) -> DateTime {
        DateTime { year, month, day, hour, minute, second, millisecond }
    }

    pub fn now() -> DateTime {
        // http://howardhinnant.github.io/date_algorithms.html.
        let (timestamp_seconds, subsec_millis) = get_unix_timestamp();

        let z = ((timestamp_seconds as i64) / 86400) + 719468;
        let era = (if z >= 0 { z } else { z - 146096 }) / 146097;
        let doe = (z - era * 146097) as u64;
        let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
        let y = yoe as i64 + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;

        let day = (doy - (153 * mp + 2) / 5 + 1) as u8;
        let month = (if mp < 10 { mp + 3 } else { mp - 9 }) as u8;
        let year = (y + i64::from(month <= 2)) as u16;
        let hour = ((timestamp_seconds / 3600) % 24) as u8;
        let minute = ((timestamp_seconds / 60) % 60) as u8;
        let second = (timestamp_seconds % 60) as u8;

        DateTime::new(year, month, day, hour, minute, second, subsec_millis)
    }
}

pub fn get_unix_timestamp() -> (u64, u32) {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    (timestamp.as_secs(), timestamp.subsec_millis())
}
