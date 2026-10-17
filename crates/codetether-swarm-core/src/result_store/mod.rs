//! Shared result publication, lookup and notification service for swarm tasks.
//!
//! [ResultStore] indexes [SharedResult] values and broadcasts [ResultNotification]
//! payloads. [SubTaskStoreHandle] implements [ResultStoreContext] for one producer.
//! [SubscriptionPattern] supports explicit filtering; expiry cleanup is caller-driven.
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{RwLock, broadcast};
mod entry;
pub use entry::{ResultNotification, SharedResult};
mod schema;
pub use schema::ResultSchema;
mod pattern;
pub use pattern::SubscriptionPattern;
mod context;
pub use context::ResultStoreContext;
mod handle;
pub use handle::SubTaskStoreHandle;
mod cleanup;
mod construct;
mod filter;
mod publish;
mod query;
mod subscription;
#[cfg(test)]
mod tests;

/// Shared result store for sub-agent communication
///
/// # Examples
/// ```
/// let store = codetether_swarm_core::result_store::ResultStore::new();
/// let _notifications = store.subscribe();
/// ```
pub struct ResultStore {
    /// All stored results by key
    results: RwLock<HashMap<String, SharedResult>>,

    /// Broadcast channel for result notifications
    notification_tx: broadcast::Sender<ResultNotification>,

    /// Subscriptions by pattern (for efficient filtering)
    subscriptions: RwLock<HashMap<String, Vec<SubscriptionPattern>>>,
}
