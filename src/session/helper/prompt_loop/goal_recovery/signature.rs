//! Stable categories for repeated transient provider failures.

pub(super) fn of(error: &anyhow::Error) -> String {
    let text = error.to_string().to_ascii_lowercase();
    [
        (["rate limit", "429"].as_slice(), "rate_limit"),
        (["timeout", "timed out", "idle"].as_slice(), "timeout"),
        (
            ["unavailable", "503", "overloaded"].as_slice(),
            "unavailable",
        ),
        (
            ["connection", "reset", "closed", "broken pipe"].as_slice(),
            "connection",
        ),
    ]
    .into_iter()
    .find_map(|(markers, name)| {
        markers
            .iter()
            .any(|item| text.contains(item))
            .then_some(name)
    })
    .unwrap_or("retryable_provider")
    .into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn ignores_changing_request_ids() {
        let a = anyhow::anyhow!("service unavailable; request 123");
        let b = anyhow::anyhow!("service unavailable; request 456");
        assert_eq!(super::of(&a), super::of(&b));
    }
}
