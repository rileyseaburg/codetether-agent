//! Load and normalize CLI-independent moonshot alignment inputs.
//!
//! [load] accepts goals and an optional JSON or line-based file, removes
//! case-insensitive duplicates and constructs a bounded [MoonshotRubric].
use crate::MoonshotRubric;
use anyhow::{Context, Result};
use std::collections::HashSet;
/// Load moonshot goals from explicit values and an optional file.
///
/// # Arguments
/// `moonshots` supplies goals; `moonshot_file` optionally appends file goals.
/// `required` enables filtering; `min_alignment` is clamped to zero through one.
/// # Returns
/// A rubric with trimmed, deduplicated goals in first-seen order.
/// # Errors
/// Returns an error when the supplied file cannot be read.
/// # Examples
/// ```rust,no_run
/// async fn example() -> anyhow::Result<()> {
/// let rubric = codetether_forage_core::rubric_load::load(
///     &["goal".into(), "GOAL".into()], None, true, 0.5).await?;
/// assert_eq!(rubric.goals, vec!["goal"]);
/// Ok(())
/// }
/// ```

pub async fn load(
    moonshots: &[String],
    moonshot_file: Option<&std::path::Path>,
    required: bool,
    min_alignment: f64,
) -> Result<MoonshotRubric> {
    let mut goals = moonshots
        .iter()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    if let Some(path) = moonshot_file {
        let content = tokio::fs::read_to_string(path)
            .await
            .with_context(|| format!("Failed to read moonshot file: {}", path.display()))?;

        if let Ok(json_list) = serde_json::from_str::<Vec<String>>(&content) {
            goals.extend(
                json_list
                    .into_iter()
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty()),
            );
        } else {
            goals.extend(
                content
                    .lines()
                    .map(str::trim)
                    .filter(|line| !line.is_empty() && !line.starts_with('#'))
                    .map(ToString::to_string),
            );
        }
    }

    let mut deduped = Vec::new();
    let mut seen = HashSet::new();
    for goal in goals {
        let key = goal.to_ascii_lowercase();
        if seen.insert(key) {
            deduped.push(goal);
        }
    }

    Ok(MoonshotRubric {
        goals: deduped,
        required: required,
        min_alignment: min_alignment.clamp(0.0, 1.0),
    })
}
