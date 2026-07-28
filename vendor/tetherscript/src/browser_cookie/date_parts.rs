//! Token parsers for cookie expiry dates.

pub(crate) fn parse_day(value: &str) -> Option<i64> {
    value
        .parse::<i64>()
        .ok()
        .filter(|day| (1..=31).contains(day))
}

pub(crate) fn parse_year(value: &str) -> Option<i64> {
    value.parse::<i64>().ok().filter(|year| *year >= 1601)
}

pub(crate) fn parse_time(value: &str) -> Option<(i64, i64, i64)> {
    let parts: Vec<_> = value.split(':').collect();
    match parts.as_slice() {
        [hour, minute, second] => Some((
            hour.parse().ok()?,
            minute.parse().ok()?,
            second.parse().ok()?,
        )),
        _ => None,
    }
}

pub(crate) fn month_number(value: &str) -> Option<i64> {
    const MONTHS: [&str; 12] = [
        "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
    ];
    MONTHS
        .iter()
        .position(|month| value.to_ascii_lowercase().starts_with(month))
        .map(|index| index as i64 + 1)
}
