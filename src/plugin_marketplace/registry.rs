//! Plugin registry — searchable catalog of community MCP tools.

use serde::{Deserialize, Serialize};

/// A plugin entry in the marketplace registry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginEntry {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub sha256: String,
    pub signature: String,
    pub download_url: String,
    pub capabilities: Vec<String>,
    pub downloads: u64,
    pub rating: f32,
    pub repo_url: Option<String>,
}

/// Search the plugin registry by name or capability.
pub fn search_plugins<'a>(query: &str, plugins: &'a [PluginEntry]) -> Vec<&'a PluginEntry> {
    let lower = query.to_lowercase();
    plugins.iter()
        .filter(|p| {
            p.name.to_lowercase().contains(&lower)
            || p.description.to_lowercase().contains(&lower)
            || p.capabilities.iter().any(|c| c.to_lowercase().contains(&lower))
        })
        .collect()
}

/// Sort plugins by popularity (downloads + rating).
pub fn by_popularity(plugins: &mut [&PluginEntry]) {
    plugins.sort_by(|a, b| {
        let score_a = a.downloads as f32 * a.rating;
        let score_b = b.downloads as f32 * b.rating;
        score_b.partial_cmp(&score_a).unwrap()
    });
}
