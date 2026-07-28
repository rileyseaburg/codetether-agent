use super::civil;

const DAY_MS: i64 = 86_400_000;

#[derive(Clone, Copy)]
pub(super) struct Parts {
    pub year: i64,
    pub month: i64,
    pub day: i64,
    pub hour: i64,
    pub minute: i64,
    pub second: i64,
    pub millisecond: i64,
}

impl Parts {
    pub(super) fn from_ms(ms: f64) -> Self {
        let ms = if ms.is_finite() { ms.trunc() as i64 } else { 0 };
        let days = ms.div_euclid(DAY_MS);
        let day_ms = ms.rem_euclid(DAY_MS);
        let (year, month, day) = civil::civil_from_days(days);
        Self {
            year,
            month: month - 1,
            day,
            hour: day_ms / 3_600_000,
            minute: day_ms % 3_600_000 / 60_000,
            second: day_ms % 60_000 / 1_000,
            millisecond: day_ms % 1_000,
        }
    }

    pub(super) fn to_ms(self) -> f64 {
        let days = civil::days_from_civil(self.year, self.month + 1, self.day);
        let time =
            self.hour * 3_600_000 + self.minute * 60_000 + self.second * 1_000 + self.millisecond;
        (days * DAY_MS + time) as f64
    }

    pub(super) fn iso(self) -> String {
        format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z",
            self.year,
            self.month + 1,
            self.day,
            self.hour,
            self.minute,
            self.second,
            self.millisecond
        )
    }
}
