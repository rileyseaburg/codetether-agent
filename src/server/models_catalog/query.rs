//! Query parameters for model catalog endpoints.

use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub(crate) struct ModelsQuery {
    pub(crate) format: Option<String>,
}
