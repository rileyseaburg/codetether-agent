//! Forage and swarm setting normalization helpers.
//!
//! The worker accepts multiple metadata spellings for execution engines and
//! swarm decomposition strategies. This module keeps those mappings local.
//!
//! # Examples
//!
//! ```ignore
//! let strategy = parse_swarm_strategy(&metadata);
//! ```

use crate::swarm::DecompositionStrategy;

use super::metadata_strings::metadata_str;

/// Normalizes forage execution-engine metadata to a supported value.
///
/// Unknown or empty values fall back to `run`.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(normalize_forage_execution_engine(Some("go".to_string())), "go");
/// ```
pub(super) fn normalize_forage_execution_engine(value: Option<String>) -> String {
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
/// ```ignore
/// assert_eq!(normalize_forage_swarm_strategy(Some("stage".to_string())), "stage");
/// ```
pub(super) fn normalize_forage_swarm_strategy(value: Option<String>) -> String {
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

/// Parses the swarm decomposition strategy from task metadata.
///
/// Several legacy key names are supported so older queue payloads still route
/// correctly.
///
/// # Examples
///
/// ```ignore
/// let strategy = parse_swarm_strategy(&metadata);
/// ```
pub(super) fn parse_swarm_strategy(
    metadata: &serde_json::Map<String, serde_json::Value>,
) -> DecompositionStrategy {
    match metadata_str(
        metadata,
        &[
            "decomposition_strategy",
            "swarm_strategy",
            "strategy",
            "swarm_decomposition",
        ],
    )
    .as_deref()
    .map(str::to_ascii_lowercase)
    .as_deref()
    {
        Some("none") | Some("single") => DecompositionStrategy::None,
        Some("domain") | Some("by_domain") => DecompositionStrategy::ByDomain,
        Some("data") | Some("by_data") => DecompositionStrategy::ByData,
        Some("stage") | Some("by_stage") => DecompositionStrategy::ByStage,
        _ => DecompositionStrategy::Automatic,
    }
}
