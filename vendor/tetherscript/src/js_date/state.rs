use super::parts::Parts;

pub(super) struct DateState {
    ms: f64,
}

impl DateState {
    pub(super) fn new(ms: f64) -> Self {
        Self { ms }
    }

    pub(super) fn ms(&self) -> f64 {
        self.ms
    }

    pub(super) fn parts(&self) -> Parts {
        Parts::from_ms(self.ms)
    }

    pub(super) fn set(&mut self, field: impl FnOnce(&mut Parts)) -> f64 {
        let mut parts = self.parts();
        field(&mut parts);
        self.ms = parts.to_ms();
        self.ms
    }
}
