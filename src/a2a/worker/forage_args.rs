//! Forage argument construction from worker task metadata.
//!
//! This module translates queue metadata into the strongly typed CLI settings
//! used by the forage executor.
//!
//! # Examples
//!
//! ```ignore
//! let args = build_forage_args("Prompt", "Title", &metadata, None);
//! ```

use std::path::PathBuf;

use crate::cli::ForageArgs;

use super::forage_settings::{normalize_forage_execution_engine, normalize_forage_swarm_strategy};
use super::metadata_flags::metadata_bool;
use super::metadata_numbers::{metadata_f64, metadata_u64, metadata_usize};
use super::metadata_strings::{metadata_str, metadata_string_list};

/// Builds `ForageArgs` from task prompt, title, and metadata.
///
/// Missing moonshots fall back to the prompt text, then the task title.
///
/// # Examples
///
/// ```ignore
/// let args = build_forage_args("Prompt", "Title", &metadata, None);
/// assert!(!args.moonshots.is_empty());
/// ```
pub(super) fn build_forage_args(
    prompt: &str,
    title: &str,
    metadata: &serde_json::Map<String, serde_json::Value>,
    selected_model: Option<String>,
) -> ForageArgs {
    let mut moonshots = metadata_string_list(
        metadata,
        &["moonshots", "moonshot", "goals", "mission", "missions"],
    );
    if moonshots.is_empty() {
        moonshots.push(prompt.trim().or_else_if_empty(title.trim()));
    }
    ForageArgs {
        top: metadata_usize(metadata, &["top"]).unwrap_or(3),
        loop_mode: metadata_bool(metadata, &["loop", "loop_mode"]).unwrap_or(false),
        interval_secs: metadata_u64(metadata, &["interval_secs", "interval"]).unwrap_or(120),
        max_cycles: metadata_usize(metadata, &["max_cycles"]).unwrap_or(0),
        execute: metadata_bool(metadata, &["execute"]).unwrap_or(false),
        no_s3: metadata_bool(metadata, &["no_s3", "local_only"]).unwrap_or(true),
        moonshots,
        moonshot_file: metadata_str(metadata, &["moonshot_file"]).map(PathBuf::from),
        moonshot_required: metadata_bool(metadata, &["moonshot_required"]).unwrap_or(false),
        moonshot_min_alignment: metadata_f64(metadata, &["moonshot_min_alignment"]).unwrap_or(0.10),
        execution_engine: normalize_forage_execution_engine(metadata_str(
            metadata,
            &["execution_engine", "engine"],
        )),
        run_timeout_secs: metadata_u64(metadata, &["run_timeout_secs", "timeout_secs", "timeout"])
            .unwrap_or(900),
        fail_fast: metadata_bool(metadata, &["fail_fast"]).unwrap_or(false),
        swarm_strategy: normalize_forage_swarm_strategy(metadata_str(
            metadata,
            &["swarm_strategy", "strategy"],
        )),
        swarm_max_subagents: metadata_usize(metadata, &["swarm_max_subagents"]).unwrap_or(8),
        swarm_max_steps: metadata_usize(metadata, &["swarm_max_steps"]).unwrap_or(100),
        swarm_subagent_timeout_secs: metadata_u64(metadata, &["swarm_subagent_timeout_secs"])
            .unwrap_or(300),
        model: selected_model,
        json: metadata_bool(metadata, &["json"]).unwrap_or(false),
    }
}

trait NonEmptyFallback {
    fn or_else_if_empty(self, fallback: &str) -> String;
}

impl NonEmptyFallback for &str {
    fn or_else_if_empty(self, fallback: &str) -> String {
        if self.is_empty() {
            fallback.to_string()
        } else {
            self.to_string()
        }
    }
}
