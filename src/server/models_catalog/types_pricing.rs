//! Pricing response struct.

use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct Pricing {
    pub(crate) prompt: String,
    pub(crate) completion: String,
}
