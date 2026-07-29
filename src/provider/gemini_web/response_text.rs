//! Selection of the authoritative text candidate from Gemini wire frames.

#[path = "response_text/placeholder.rs"]
pub mod placeholder;

use serde_json::Value;

pub(super) fn latest(raw: &str) -> String {
    let mut latest = String::new();
    for line in raw
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with('['))
    {
        let Ok(events) = serde_json::from_str::<Value>(line) else {
            continue;
        };
        let Some(events) = events.as_array() else {
            continue;
        };
        for event in events {
            if let Some(text) = event_text(event)
                && !text.is_empty()
                && !placeholder::is_placeholder(&text)
            {
                latest = text;
            }
        }
    }
    latest
}

fn event_text(event: &Value) -> Option<String> {
    let inner = event.get(2)?.as_str()?;
    if !inner.starts_with('[') {
        return None;
    }
    let value = serde_json::from_str::<Value>(inner).ok()?;
    value
        .get(4)?
        .get(0)?
        .get(1)?
        .get(0)?
        .as_str()
        .map(str::to_string)
}

#[cfg(test)]
#[path = "response_text_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "response_text/placeholder_tests.rs"]
mod placeholder_tests;
