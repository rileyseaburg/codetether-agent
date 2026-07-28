use super::parts::Parts;

#[derive(Clone, Copy)]
pub(super) enum Field {
    Year,
    Month,
    Date,
    Hours,
    Minutes,
    Seconds,
    Milliseconds,
}

pub(super) fn read(field: Field, parts: Parts) -> i64 {
    match field {
        Field::Year => parts.year,
        Field::Month => parts.month,
        Field::Date => parts.day,
        Field::Hours => parts.hour,
        Field::Minutes => parts.minute,
        Field::Seconds => parts.second,
        Field::Milliseconds => parts.millisecond,
    }
}

pub(super) fn write(field: Field, parts: &mut Parts, value: i64) {
    match field {
        Field::Year => parts.year = value,
        Field::Month => parts.month = value,
        Field::Date => parts.day = value,
        Field::Hours => parts.hour = value,
        Field::Minutes => parts.minute = value,
        Field::Seconds => parts.second = value,
        Field::Milliseconds => parts.millisecond = value,
    }
}
