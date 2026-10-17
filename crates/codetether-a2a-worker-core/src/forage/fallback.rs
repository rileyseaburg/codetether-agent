//! Nonempty text fallback for task mission selection.

pub(super) trait NonEmptyFallback {
    fn or_else_if_empty(self, fallback: &str) -> String;
}

impl NonEmptyFallback for &str {
    fn or_else_if_empty(self, fallback: &str) -> String {
        if self.is_empty() {
            fallback.to_string()
        } else {
            self.to_string()
        }
    }
}
