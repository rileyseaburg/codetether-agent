//! Normalize presentation-only ANSI styling for process-proof assertions.

use regex::Regex;
use std::sync::LazyLock;

static SGR: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\x1b\[[0-9;]*m").expect("valid ANSI SGR pattern"));

pub(super) fn plain(log: &str) -> String {
    SGR.replace_all(log, "").into_owned()
}

#[cfg(test)]
mod tests {
    use super::plain;

    #[test]
    fn normalizes_colored_tracing_fields_without_changing_raw_text() {
        let raw = "Discovered A2A peer \x1b[3mpeer_name\x1b[0m\x1b[2m=\x1b[0mbeta \
                   \x1b[3mendpoint\x1b[0m\x1b[2m=\x1b[0mhttp://localhost:1234\n\
                   Auto-intro message sent \x1b[3mpeer\x1b[0m\x1b[2m=\x1b[0mhttp://localhost:1234\n";
        let expected = "Discovered A2A peer peer_name=beta endpoint=http://localhost:1234\n\
                        Auto-intro message sent peer=http://localhost:1234\n";
        assert_eq!(plain(raw), expected);
        assert_eq!(plain(expected), expected);
        assert!(raw.contains("\x1b[3mpeer_name"));
    }
}
