//! Compatibility exports for shared result storage and task-scoped handles.
//!
//! Use [ResultStore] directly or publish through [SubTaskStoreHandle].
pub use codetether_swarm_core::result_store::{
    ResultNotification, ResultSchema, ResultStore, ResultStoreContext, SharedResult,
    SubTaskStoreHandle, SubscriptionPattern,
};
