//! Decoded lifecycle target and injected runtime context.
use crate::RuntimeContext;
use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct Args {
    pub(super) target: String,
    #[serde(flatten)]
    pub(super) context: RuntimeContext,
}
