//! CodeTether Agent - A2A-native AI coding agent
//!
//! A Rust implementation of an AI coding agent with first-class support for
//! the A2A (Agent-to-Agent) protocol and the CodeTether ecosystem.

pub mod a2a;
pub mod agent;
pub mod audit;
pub mod autochat;
pub mod benchmark;
pub mod browser;
pub mod bus;
pub mod cli;
pub mod cloudevents;
pub mod cognition;
pub mod config;
pub mod crash;
pub mod distill;
pub mod event_stream;
pub mod forage;
pub mod github_pr;
pub mod image_clipboard;
pub mod indexer;
pub mod k8s;
pub mod knowledge_graph;
pub mod lsp;
pub mod marketplace;
pub mod mcp;
pub mod memory;
pub mod mesh;
pub mod moltbook;
pub mod okr;
pub mod platform;
pub mod plugin_marketplace;
pub mod provenance;
pub mod provider;
pub mod ralph;
pub mod rlm;
pub mod search;
pub mod secrets;
pub mod server;
pub mod session;
pub mod swarm;
pub mod telemetry;
pub mod tls;
pub mod tool;
pub mod tui;
pub mod util;
pub mod voice;
pub mod worker_server;
pub mod workspace_scan;
pub mod worktree;
pub mod worktree_stub;
