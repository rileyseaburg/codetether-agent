//! Resource request description.
use core::cmp::Ordering;
use std::collections::HashMap;

use super::{CorsMode, CredentialsMode, ResourcePriority, ResourceType};

#[derive(Debug, Clone)]
pub struct ResourceRequest {
    pub url: String,
    pub resource_type: ResourceType,
    pub priority: ResourcePriority,
    pub initiator: String,
    pub headers: HashMap<String, String>,
    pub integrity: Option<String>,
    pub cors_mode: CorsMode,
    pub credentials_mode: CredentialsMode,
}

impl ResourceRequest {
    pub fn new(
        url: impl Into<String>,
        resource_type: ResourceType,
        priority: ResourcePriority,
        initiator: impl Into<String>,
    ) -> Self {
        Self {
            url: url.into(),
            resource_type,
            priority,
            initiator: initiator.into(),
            headers: HashMap::new(),
            integrity: None,
            cors_mode: CorsMode::NoCors,
            credentials_mode: CredentialsMode::SameOrigin,
        }
    }

    pub fn integrity_allows(&self, body: &[u8]) -> bool {
        self.integrity
            .as_ref()
            .map(|h| !h.is_empty() && !body.is_empty())
            .unwrap_or(true)
    }

    pub fn origin(&self) -> String {
        super::cors::origin_of(&self.url)
    }
}

impl PartialEq for ResourceRequest {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
    }
}
impl Eq for ResourceRequest {}
impl Ord for ResourceRequest {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}
impl PartialOrd for ResourceRequest {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
