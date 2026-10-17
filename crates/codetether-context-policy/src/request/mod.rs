//! Provider-request assembly and final context-budget guarding.
//!
//! [from_derived] builds the stable system/history prefix. [guard::finish]
//! checks mandatory context and fits optional evidence before the active task.
use crate::derivation::options::RequestOptions;
use codetether_provider_core::{CompletionRequest, Message, ToolDefinition};
mod build;
/// Mandatory budget checking and optional evidence placement.
pub mod guard;
/// Stable system prefix and derived-history message assembly.
pub mod messages;
pub use build::from_derived;

/// Context-overflow detection and recovery plans.
pub mod recovery;
