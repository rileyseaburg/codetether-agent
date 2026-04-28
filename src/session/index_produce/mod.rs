//! RLM-backed summary production for [`SummaryIndex::summary_for`].
//!
//! This module is the "producer" half of step 18. The index data
//! structure lives in [`super::index`]; this file owns the async
//! call through the RLM router to materialise a summary.

mod build_context;
mod call;
mod input;

use std::sync::Arc;

use anyhow::{Context as _, Result};
use tracing::info;

use super::index::types::{Granularity, SummaryNode, SummaryRange};
use crate::provider::{Message, Provider};
use crate::rlm::RlmConfig;

pub use call::produce_summary;
