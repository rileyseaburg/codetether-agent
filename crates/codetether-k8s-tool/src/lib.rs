//! Kubernetes tool schema, validation and scoped action dispatch.
//!
//! [K8sTool] lazily initializes or reuses a shared manager. Individual action
//! modules preserve manager scoping and the existing tool result contracts.
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use codetether_k8s::K8sManager;
use codetether_tool_core::{Tool, ToolResult};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::OnceCell;
mod construct;
mod delete_pod;
mod input;
mod list_pods;
mod logs;
mod pod_state;
mod recent_actions;
mod rolling_restart;
mod scale;
mod schema;
mod schema_fields;
mod spawn_pod;
mod status;
mod tool;
pub use construct::K8sTool;
use input::{K8sAction, K8sInput};
