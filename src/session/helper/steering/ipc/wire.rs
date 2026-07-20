//! Local steering wire messages.

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(super) struct Request {
    pub(super) text: String,
}

#[derive(Deserialize, Serialize)]
pub(super) struct Reply {
    pub(super) accepted: bool,
}
