//! Mark and measure mutation helpers.

use super::*;

pub(super) fn add_mark(name: String) -> PerformanceEntry {
    store::with_mut(|timeline| {
        let entry = PerformanceEntry::mark(name, store::tick(timeline));
        timeline.entries.push(entry.clone());
        entry
    })
}

pub(super) fn add_measure(
    name: String,
    start: Option<String>,
    end: Option<String>,
) -> Result<PerformanceEntry, String> {
    store::with_mut(|timeline| {
        let start_time = mark_time(&timeline.entries, start.as_deref())?;
        let end_time = end
            .as_deref()
            .map(|name| mark_time(&timeline.entries, Some(name)))
            .unwrap_or_else(|| Ok(store::tick(timeline)))?;
        let entry = PerformanceEntry::measure(name, start_time, end_time);
        timeline.entries.push(entry.clone());
        Ok(entry)
    })
}

fn mark_time(entries: &[PerformanceEntry], name: Option<&str>) -> Result<f64, String> {
    let Some(name) = name else {
        return Ok(0.0);
    };
    entries
        .iter()
        .rev()
        .find(|entry| entry.entry_type == "mark" && entry.name == name)
        .map(|entry| entry.start_time)
        .ok_or_else(|| format!("performance.measure: mark '{}' was not found", name))
}
