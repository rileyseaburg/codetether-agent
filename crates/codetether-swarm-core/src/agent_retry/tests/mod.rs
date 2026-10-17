//! Retry stop/count behavior using in-memory attempts.
use super::{AgentAttempt, AgentLoopExit, AgentOutcome, run};
use async_trait::async_trait;
use std::sync::atomic::{AtomicUsize, Ordering};
mod fixture;
use fixture::Attempt;
mod complete;
mod exhausted;
mod recovery;
