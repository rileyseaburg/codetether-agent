//! Discover JSON benchmark PRDs and apply the configured tier filter.
use crate::{BenchmarkConfig, detect_tier};
use std::path::{Path, PathBuf};

/// Discover benchmark PRD files in the configured directory
///
/// # Arguments
/// * `config` - Directory and optional tier filter.
/// # Returns
/// Matching JSON paths sorted by tier.
/// # Errors
/// Returns an error for invalid input or unavailable benchmark files.
/// # Examples
/// ```no_run
/// let config = codetether_benchmark::BenchmarkConfig::default();
/// let _ = codetether_benchmark::discover_prds(&config);
/// ```
pub fn discover_prds(config: &BenchmarkConfig) -> anyhow::Result<Vec<(PathBuf, u8)>> {
    let dir = Path::new(&config.prd_dir);
    if !dir.exists() {
        anyhow::bail!("Benchmark directory not found: {}", config.prd_dir);
    }

    let mut prds = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "json") {
            let filename = path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            let tier = detect_tier(&filename);

            // Filter by tier if specified
            if let Some(filter_tier) = config.tier
                && tier != filter_tier
            {
                continue;
            }

            prds.push((path, tier));
        }
    }

    prds.sort_by_key(|(_, tier)| *tier);
    Ok(prds)
}
