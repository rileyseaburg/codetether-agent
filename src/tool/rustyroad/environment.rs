//! Explicit database environment selection, independent of ambient variables.

use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(super) enum Environment {
    #[default]
    Dev,
    Test,
    Prod,
}

impl Environment {
    pub(super) fn as_str(&self) -> &'static str {
        match self {
            Self::Dev => "dev",
            Self::Test => "test",
            Self::Prod => "prod",
        }
    }
}
