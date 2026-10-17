//! Classify log line.

pub(crate) fn is_log_line(line: &str) -> bool {
    if line.len() >= 10
        && line.chars().take(4).all(|c| c.is_ascii_digit())
        && line.chars().nth(4) == Some('-')
    {
        return true;
    }
    if line.starts_with('[')
        && line.len() > 5
        && line.chars().nth(1).is_some_and(|c| c.is_ascii_digit())
    {
        return true;
    }
    let lvls = ["INFO", "DEBUG", "WARN", "ERROR", "FATAL", "TRACE"];
    lvls.iter()
        .any(|l| line.starts_with(l) || line.contains(&format!(" {} ", l)))
}
