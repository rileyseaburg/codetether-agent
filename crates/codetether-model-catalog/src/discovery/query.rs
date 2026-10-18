//! Query parameters for model catalog endpoints.

use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct ModelsQuery {
    pub format: Option<String>,
}
