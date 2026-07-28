pub(super) fn validate(name: &str) -> Result<(), String> {
    let normalized = name.to_ascii_lowercase();
    if matches!(
        normalized.as_str(),
        "event" | "events" | "htmlevents" | "mouseevent" | "mouseevents" | "customevent"
    ) {
        return Ok(());
    }
    Err(format!(
        "document.createEvent unsupported event interface: {}",
        name
    ))
}
