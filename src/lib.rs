//! CodeTether Agent - A2A-native AI coding agent
//!
//! A Rust implementation of an AI coding agent with first-class support for the
//! A2A (Agent-to-Agent) protocol and the CodeTether ecosystem.

pub mod a2a;
pub mod agent;
pub mod audit;
pub mod benchmark;
pub mod bus;
pub mod cli;
pub mod cognition;
pub mod config;
pub mod event_stream;
pub mod k8s;
pub mod lsp;
pub mod mcp;
pub mod moltbook;
pub mod okr;
pub mod opencode;
pub mod provider;
pub mod ralph;
pub mod rlm;
pub mod secrets;
pub mod server;
pub mod session;
pub mod swarm;
pub mod telemetry;
pub mod tool;
pub mod tui;
pub mod worker_server;
pub mod worktree;
