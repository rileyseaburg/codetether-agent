use serde::{Deserialize, Serialize};

/// Addressable actor identity used for mailbox routing.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ActorId(pub String);

impl ActorId {
    /// Creates a new actor id from any string-like value.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the bus topic used as this actor's mailbox.
    pub fn inbox_topic(&self) -> String {
        format!("actor.{}.inbox", self.0)
    }
}

impl std::fmt::Display for ActorId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<&str> for ActorId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for ActorId {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}
