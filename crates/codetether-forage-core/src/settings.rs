//! Normalization of worker-supplied forage execution settings.
//!
//! Engine names and strategy names are normalized independently, with the
//! existing run/auto fallbacks for unknown values.

/// Normalizes forage execution-engine metadata to a supported value.
///
/// Unknown or empty values fall back to `run`.
///
/// # Examples
///
/// ```rust
/// use codetether_forage_core::settings::normalize_forage_execution_engine;
/// assert_eq!(normalize_forage_execution_engine(Some("go".to_string())), "go");
/// ```
/// # Arguments
/// * `value` - Optional engine or strategy name from task metadata.
/// # Returns
/// The supported lowercase name, or the documented default.
pub fn normalize_forage_execution_engine(value: Option<String>) -> String {
    match value
        .as_deref()
        .unwrap_or("run")
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "swarm" => "swarm".to_string(),
        "go" => "go".to_string(),
        _ => "run".to_string(),
    }
}

/// Normalizes forage swarm-strategy metadata to a supported value.
///
/// Unknown or empty values fall back to `auto`.
///
/// # Examples
///
/// ```rust
/// use codetether_forage_core::settings::normalize_forage_swarm_strategy;
/// assert_eq!(normalize_forage_swarm_strategy(Some("stage".to_string())), "stage");
/// ```
/// # Arguments
/// * `value` - Optional engine or strategy name from task metadata.
/// # Returns
/// The supported lowercase name, or the documented default.
pub fn normalize_forage_swarm_strategy(value: Option<String>) -> String {
    match value
        .as_deref()
        .unwrap_or("auto")
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "domain" | "data" | "stage" | "none" => value.unwrap().trim().to_ascii_lowercase(),
        _ => "auto".to_string(),
    }
}
