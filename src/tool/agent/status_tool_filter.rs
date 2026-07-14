//! Ownership filtering for tool-activity status signals.

use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};

pub(super) fn owned<'a>(
    activity: HashMap<String, DateTime<Utc>>,
    allowed: impl Iterator<Item = &'a String>,
) -> HashMap<String, DateTime<Utc>> {
    let allowed: HashSet<&str> = allowed.map(String::as_str).collect();
    activity
        .into_iter()
        .filter(|(agent, _)| allowed.contains(agent.as_str()))
        .collect()
}
