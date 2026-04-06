//! Terminal User Interface
//!
//! Interactive TUI using Ratatui
//!
//! Module organization:
//! - `constants` — Configuration values and limits
//! - `types` — Data types (App, ViewMode, MessageType, etc.)
//! - `entry` — Terminal setup/teardown and `run()` entry point
//! - `workspace` — Workspace snapshot and git detection
//! - `relay_checkpoint` — Relay checkpoint persistence and OKR approval
//! - `chat_sync` — Chat archive synchronization to S3/MinIO
//! - `smart_switch` — Smart provider switching and retry logic
//! - `commands` — Slash command parsing and hints
//! - `format` — Formatting utilities, agent identity, clipboard
//! - `autochat_worker` — Autochat relay workers and planning
//! - `app_impl` — App struct implementation
//! - `event_loop` — Main TUI event loop
//! - `render` — All rendering functions
//! - `tests` — Unit tests

pub mod bus_log;
pub mod message_formatter;
pub mod ralph_view;
pub mod swarm_view;
pub mod theme;
pub mod theme_utils;
pub mod token_display;
pub mod worker_bridge;

// Refactored sub-modules
mod constants;
mod types;
mod entry;
mod workspace;
mod relay_checkpoint;
mod chat_sync;
mod smart_switch;
mod commands;
mod format;
mod autochat_worker;
mod app_impl;
mod event_loop;
mod render;
#[cfg(test)]
mod tests;

// Re-export public API
pub use entry::run;

// Shared imports — these are inherited by sub-modules via `use super::*`
use crate::autochat::shared_context::{
    SharedRelayContext, compose_prompt_with_context, distill_context_delta_with_rlm,
    drain_context_updates, publish_context_delta,
};
use crate::autochat::transport::{attach_handoff_receiver, consume_handoff_by_correlation};
use crate::autochat::{
    model_rotation::RelayModelRotation, model_rotation::build_round_robin_model_rotation,
};
use crate::bus::relay::{ProtocolRelayRuntime, RelayAgentProfile};
use crate::config::Config;
use crate::okr::{
    ApprovalDecision, KeyResult, KrOutcome, KrOutcomeType, Okr, OkrRepository, OkrRun, OkrRunStatus,
};
use crate::provider::{ContentPart, Role};
use crate::ralph::{RalphConfig, RalphLoop};
use crate::rlm::{FinalPayload, RlmExecutor};
use crate::session::{Session, SessionEvent, SessionSummary, list_sessions_paged};
use crate::swarm::{DecompositionStrategy, SwarmConfig, SwarmExecutor};
use crate::tui::bus_log::{BusLogState, render_bus_log};
use crate::tui::message_formatter::MessageFormatter;
use crate::tui::ralph_view::{RalphEvent, RalphViewState, render_ralph_view};
use crate::tui::swarm_view::{SwarmEvent, SwarmViewState, render_swarm_view};
use crate::tui::theme::Theme;
use crate::tui::token_display::TokenDisplay;
use crate::tui::worker_bridge::{TuiWorkerBridge, WorkerBridgeCmd};
use anyhow::Result;
use base64::Engine;
use crossterm::{
    event::{
        DisableBracketedPaste, EnableBracketedPaste, Event, EventStream, KeyCode, KeyEventKind,
        KeyModifiers,
    },
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use futures::StreamExt;
use minio::s3::builders::ObjectContent;
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use minio::s3::types::S3Api;
use minio::s3::{Client as MinioClient, ClientBuilder as MinioClientBuilder};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, Clear, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap,
    },
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use uuid::Uuid;
