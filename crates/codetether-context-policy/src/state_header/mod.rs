//! Compression-safe formatting for hard-pinned constraints.
//!
//! [prepend_state_header] combines persisted prefix lines with live pin
//! excerpts, retaining the original cap and source order.
use crate::pages::PageKind;
use codetether_provider_core::{ContentPart, Message, Role};

const MAX_CONSTRAINTS: usize = 8;

const EXCERPT_CHARS: usize = 120;
mod apply;
pub use apply::prepend_state_header;
mod constraints;
use constraints::pinned_constraints;
mod excerpt;
use excerpt::first_text_excerpt;
