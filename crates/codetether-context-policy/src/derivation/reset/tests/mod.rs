//! Reset branch and provenance regression tests.
use super::*;
use codetether_provider_core::{ContentPart, Role};
fn message(role: Role, text: &str) -> Message {
    Message {
        role,
        content: vec![ContentPart::Text { text: text.into() }],
    }
}
mod anchored_marker;
mod below_threshold;
mod legacy;
mod marker_without_user;
mod no_prefix;
mod summary_plan;
