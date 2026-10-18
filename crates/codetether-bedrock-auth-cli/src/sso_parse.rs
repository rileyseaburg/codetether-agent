//! Tiny INI parser for AWS config SSO profile lookup.

use std::collections::HashMap;

/// Parsed AWS config sections.
pub(super) type Sections = HashMap<String, HashMap<String, String>>;

/// Parse `[section]` and `key = value` lines from AWS config.
pub(super) fn parse_sections(text: &str) -> Sections {
    let mut map = Sections::new();
    let mut current = String::new();
    for line in text.lines().map(str::trim) {
        if let Some(name) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            current = name.to_string();
        } else if let Some((k, v)) = line.split_once('=') {
            map.entry(current.clone())
                .or_default()
                .insert(k.trim().into(), v.trim().into());
        }
    }
    map
}
