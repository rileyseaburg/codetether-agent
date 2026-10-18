//! Thought prompts, output normalization and deterministic context summaries.
//!
//! [build_phase_prompts] formats shared records for a model call. Normalize
//! replies with [normalize_thought_output]; unusable replies use
//! [phase_default_text]. These helpers perform no model or tool execution.
use crate::text as text_util;
use crate::{ThoughtEvent, ThoughtEventType, ThoughtPhase, ThoughtWorkItem};
mod context_signals;
mod context_summary;
mod normalize;
mod normalize_shape;
mod phase_default;
mod placeholder;
mod prompt_build;
mod prompt_phase;
#[cfg(test)]
mod tests;
pub use context_signals::{Signals, collect};
pub use context_summary::summarize;
pub use normalize::normalize_thought_output;
pub use normalize_shape::{labeled_section, looks_meta};
pub use phase_default::phase_default_text;
pub use placeholder::{find_process_label_start, has_template_placeholder_values};
pub use prompt_build::{build_phase_prompts, format_context_event};
pub use prompt_phase::instruction;
