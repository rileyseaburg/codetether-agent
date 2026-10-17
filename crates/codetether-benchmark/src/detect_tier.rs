//! Derive benchmark difficulty tiers from PRD filenames.

/// Detect tier from PRD filename (e.g., "t1-rest-api.json" -> 1)
///
/// # Arguments
/// * `filename` - PRD filename to classify.
/// # Returns
/// Tier 1, 2, or 3, defaulting to 2.
/// # Examples
/// ```
/// assert_eq!(codetether_benchmark::detect_tier("t1-example.json"), 1);
/// ```
pub fn detect_tier(filename: &str) -> u8 {
    if filename.starts_with("t1-") {
        1
    } else if filename.starts_with("t2-") {
        2
    } else if filename.starts_with("t3-") {
        3
    } else {
        2 // default to medium
    }
}
