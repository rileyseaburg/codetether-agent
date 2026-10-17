//! Message or list arguments with trusted runtime context.
use crate::RuntimeContext;
use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct Args {
    pub(super) target: String,
    pub(super) message: String,
    #[serde(flatten)]
    pub(super) context: RuntimeContext,
}
