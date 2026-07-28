//! Cookie expiry parsing.

use std::time::{SystemTime, UNIX_EPOCH};

use super::date_civil::days_from_civil;
use super::date_parts::{month_number, parse_day, parse_time, parse_year};

pub(crate) fn now_seconds() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs() as i64)
        .unwrap_or_default()
}

pub(crate) fn parse_cookie_time(value: &str) -> Option<i64> {
    let value = value.trim();
    if let Ok(seconds) = value.parse::<i64>() {
        return Some(seconds);
    }
    let clean: String = value
        .chars()
        .map(|ch| if ch == ',' || ch == '-' { ' ' } else { ch })
        .collect();
    let parts: Vec<&str> = clean.split_whitespace().collect();
    let day = parts.iter().find_map(|part| parse_day(part))?;
    let month = parts.iter().find_map(|part| month_number(part))?;
    let year = parts.iter().find_map(|part| parse_year(part))?;
    let (hour, minute, second) = parts.iter().find_map(|part| parse_time(part))?;
    Some(days_from_civil(year, month, day) * 86_400 + hour * 3_600 + minute * 60 + second)
}
