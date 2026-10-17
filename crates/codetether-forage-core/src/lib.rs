//! Forage opportunity scoring, prompt composition and result presentation.
//!
//! Call [collect_opportunities_with_rubric] with loaded OKRs and a scoring hook.
//! The application owns persistence, execution, CLI output and event publication.
mod alignment;
mod build;
mod collect;
mod hits;
mod opportunity;
mod prompt;
mod prompt_fields;
mod render;
mod rubric;
mod summary;
mod text;
mod tokenize;
mod weights;
pub use collect::collect_opportunities_with_rubric;
pub use opportunity::ForageOpportunity;
pub use prompt_fields::normalize_prompt_field;
pub use rubric::MoonshotRubric;
pub use summary::{ForageRunSummary, ForageSelectionSummary};
pub use text::{truncate_chars_with_suffix, truncate_goal};
pub use weights::{status_weight, urgency_bonus};

mod record;

/// Evidence-driven key-result progress updates.
pub mod progress;
/// Persistent objective seeding for forage workflows.
pub mod seed;

/// Git change snapshots for execution evidence.
pub mod changes;
/// Quality workflow with host-supplied command execution.
pub mod quality;
