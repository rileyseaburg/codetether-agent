//! Deserialization contract for `send_input`.

use super::item::InputItem;
use crate::tool::collaboration::context::RuntimeContext;
use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct Args {
    pub(super) target: String,
    #[serde(default)]
    pub(super) message: Option<String>,
    #[serde(default)]
    pub(super) items: Option<Vec<InputItem>>,
    #[serde(default)]
    pub(super) interrupt: bool,
    #[serde(flatten)]
    pub(super) context: RuntimeContext,
}
