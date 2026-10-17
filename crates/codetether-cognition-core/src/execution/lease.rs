//! Short-lived capability lease records.
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A short-lived capability granting tool access to a persona.
///
/// # Examples
/// ```
/// use codetether_cognition_core::execution::CapabilityLease;
/// let lease = CapabilityLease::new("p1", "read", "policy");
/// assert_eq!(lease.persona_id, "p1");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityLease {
    pub id: String,
    pub persona_id: String,
    pub tool_name: String,
    pub granted_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub granted_by: String,
}

impl CapabilityLease {
    /// Create a new 60-second capability lease.
    ///
    /// # Examples
    /// ```
    /// use codetether_cognition_core::execution::CapabilityLease;
    /// let lease = CapabilityLease::new("p1", "read", "policy");
    /// assert_eq!((lease.expires_at - lease.granted_at).num_seconds(), 60);
    /// ```
    pub fn new(persona_id: &str, tool_name: &str, granted_by: &str) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            persona_id: persona_id.to_string(),
            tool_name: tool_name.to_string(),
            granted_at: now,
            expires_at: now + Duration::seconds(60),
            granted_by: granted_by.to_string(),
        }
    }

    /// Check if the lease is still valid.
    ///
    /// # Examples
    /// ```
    /// use codetether_cognition_core::execution::CapabilityLease;
    /// let mut lease = CapabilityLease::new("p1", "read", "policy");
    /// lease.expires_at = lease.granted_at - chrono::Duration::seconds(1);
    /// assert!(!lease.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        Utc::now() < self.expires_at
    }
}
