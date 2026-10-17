//! One durable queued child-agent follow-up.

use serde::{Deserialize, Serialize};

use codetether_collaboration_input::MessageImage;

/// Whether the front delivery is queued or claimed by a running turn.
///
/// # Examples
/// ```
/// let state=codetether_child_mailbox::DeliveryState::Pending;
/// assert!(matches!(state,codetether_child_mailbox::DeliveryState::Pending));
/// ```
#[derive(Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
pub enum DeliveryState {
    /// Waiting to be claimed.
    Pending,
    /// Claimed by a running child turn.
    Running,
}

/// Durable delivery identity, input and parent context.
///
/// # Examples
/// ```
/// let item=codetether_child_mailbox::Item::new("follow up".into(),vec![],None,None);
/// assert!(!item.id.is_empty());
/// ```
#[derive(Clone, Deserialize, Serialize)]
pub struct Item {
    /// Stable delivery identity.
    pub id: String,
    /// Queued input text.
    pub message: String,
    #[serde(default)]
    /// Durable image attachments.
    pub images: Vec<MessageImage>,
    /// Parent session owning the submission.
    pub parent_id: Option<String>,
    /// Parent context inheritance permission.
    pub prior_context: Option<bool>,
    /// Pending or currently claimed state.
    pub state: DeliveryState,
}

impl Item {
    /// Construct a pending delivery with a fresh UUID.
    pub fn new(
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
