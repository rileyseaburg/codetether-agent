//! Serializable metrics and execution/display event data.
//!
//! Events retain transient timing fields while status and metrics keep their
//! original serde representation.
use codetether_agent_core::events::{AgentMessageEntry, AgentToolCallDetail};
use serde::{Deserialize, Serialize};
mod swarm_stats;
pub use swarm_stats::SwarmStats;
mod stage_stats;
pub use stage_stats::StageStats;
mod sub_task_status;
pub use sub_task_status::SubTaskStatus;
mod cache_stats;
pub use cache_stats::CacheStats;
mod rate_limit_info;
pub use rate_limit_info::RateLimitInfo;
mod rate_limit_stats;
pub use rate_limit_stats::RateLimitStats;
mod swarm_event;
pub use swarm_event::SwarmEvent;
mod sub_task_info;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
pub use sub_task_info::SubTaskInfo;
mod sub_task;
pub use sub_task::SubTask;
mod sub_task_context;
pub use sub_task_context::SubTaskContext;
mod sub_agent;
pub use sub_agent::SubAgent;
mod sub_agent_status;
pub use sub_agent_status::SubAgentStatus;
mod tool_call_record;
pub use tool_call_record::ToolCallRecord;
mod sub_task_result;
pub use sub_task_result::SubTaskResult;
