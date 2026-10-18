//! Worker HTTP state, task ingress, health probes and bus endpoints.
//!
//! Construct [WorkerServerState], attach its task channel and bus, then use
//! [serve] to expose the worker routes. CLI parsing and worker startup remain
//! responsibilities of the caller.
use anyhow::Result;
use axum::{
    Json, Router,
    body::Body,
    extract::State,
    http::{HeaderMap, Request, StatusCode},
    response::sse::{Event, KeepAlive, Sse},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use codetether_a2a_worker_core::{HeartbeatState, WorkerStatus};
use codetether_bus_core::{AgentBus, BusEnvelope};
use futures::stream;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::{Mutex, broadcast, mpsc};
mod bus_state;
mod cloudevents;
mod construct;
mod context;
mod events;
mod health;
mod heartbeat;
mod identity;
mod ingress;
mod publish;
mod ready;
mod serve;
mod state;
mod status;
mod stream_endpoint;
mod tasks;
mod topic;
mod wire;
use cloudevents::parse_cloud_event;
use events::events;
use health::health;
use ingress::receive_task;
use publish::publish_bus_event;
use ready::ready;
pub use serve::serve;
pub use state::WorkerServerState;
use status::worker_status;
use stream_endpoint::stream_bus_events;
use topic::bus_topic_matches;
use wire::{BusPublishRequest, HeartbeatInfo, WorkerStatusResponse};
