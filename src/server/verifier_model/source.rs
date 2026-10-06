//! Classify the source of a selection using the harness's precedence.
use super::types::Source;
use crate::tool::goal::verify::is_self_review;

pub(super) fn classify(
    runtime: Option<&str>,
    environment: Option<&str>,
    configured: Option<&str>,
    worker: Option<&str>,
) -> Source {
    if runtime.is_some() {
        Source::Runtime
    } else if environment.is_some() {
        Source::Environment
    } else if configured.is_some_and(|model| !is_self_review(model, worker)) {
        Source::Default
    } else if worker.is_some() {
        Source::Worker
    } else if configured.is_some() {
        Source::Default
    } else {
        Source::Unconfigured
    }
}
