use super::Field;

pub(super) fn getters() -> [(&'static str, Field); 7] {
    [
        ("getFullYear", Field::Year),
        ("getMonth", Field::Month),
        ("getDate", Field::Date),
        ("getHours", Field::Hours),
        ("getMinutes", Field::Minutes),
        ("getSeconds", Field::Seconds),
        ("getMilliseconds", Field::Milliseconds),
    ]
}

pub(super) fn setters() -> [(&'static str, Field); 7] {
    [
        ("setFullYear", Field::Year),
        ("setMonth", Field::Month),
        ("setDate", Field::Date),
        ("setHours", Field::Hours),
        ("setMinutes", Field::Minutes),
        ("setSeconds", Field::Seconds),
        ("setMilliseconds", Field::Milliseconds),
    ]
}
