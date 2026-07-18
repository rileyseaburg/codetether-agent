//! One durable queued child-agent follow-up.

use serde::{Deserialize, Serialize};

use crate::tool::agent::collaboration_runtime::message_input::MessageImage;

#[derive(Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
pub(super) enum DeliveryState {
    Pending,
    Running,
}

#[derive(Clone, Deserialize, Serialize)]
pub(super) struct Item {
    pub(super) id: String,
    pub(super) message: String,
    #[serde(default)]
    pub(super) images: Vec<MessageImage>,
    pub(super) parent_id: Option<String>,
    pub(super) prior_context: Option<bool>,
    pub(super) state: DeliveryState,
}

impl Item {
    pub(super) fn new(
        message: String,
        images: Vec<MessageImage>,
        parent_id: Option<String>,
        prior: Option<bool>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            message,
            images,
            parent_id,
            prior_context: prior,
            state: DeliveryState::Pending,
        }
    }
}
