//! Compression-safe formatting for hard-pinned constraints.
//!
//! [prepend_state_header] combines complete persisted and live pinned sources.
//! Supersession is explicit; active text is never truncated or silently dropped.
use crate::pages::PageKind;
use codetether_provider_core::{ContentPart, Message, Role};

mod apply;
pub use apply::{prepend_state_header, prepend_state_header_with_corrections};
mod correction;
pub use correction::ConstraintCorrection;
mod constraints;
mod corrected_text;
use constraints::pinned_constraints;
mod record;
pub use record::ConstraintRecord;
