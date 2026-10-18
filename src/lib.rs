//! CodeTether Agent - A2A-native AI coding agent
//!
//! A Rust implementation of an AI coding agent with first-class support for
//! the A2A (Agent-to-Agent) protocol and the CodeTether ecosystem.

/// Process-wide capacity-guarding allocator. Catches a single runaway
/// allocation (a corrupted capacity hint reserving tens of GiB) and aborts
/// cleanly with a spooled crash report rather than letting the OS OOM-kill
/// the process. See [`telemetry::alloc_guard`]. The guard is inert until
/// [`telemetry::alloc_guard_config::configure`] arms it at startup.
#[global_allocator]
static GLOBAL_ALLOC: telemetry::alloc_guard::GuardAlloc = telemetry::alloc_guard::GuardAlloc;

pub mod a2a;
pub mod agent;
pub use codetether_audit as audit;
pub use codetether_native_approval as approval;
pub mod autochat;
pub mod benchmark;
pub use codetether_browser::browser;
pub mod bus;
pub mod cli;
pub use codetether_task_ingest as cloudevents;
pub mod cognition;
pub use codetether_config as config;
pub mod crash;
pub use codetether_distill as distill;
pub use codetether_event_stream as event_stream;
pub mod forage;
pub mod github_pr;
pub mod image_clipboard;
pub use codetether_k8s as k8s;
pub use codetether_knowledge_graph as knowledge_graph;
pub use codetether_native_indexer as indexer;
pub mod lsp;
pub use codetether_marketplace as marketplace;
pub mod mcp;
pub use codetether_cli_startup::environment as noninteractive_env;
pub use codetether_cognition_core as memory;
pub use codetether_mesh as mesh;
pub use codetether_moltbook as moltbook;
pub use codetether_native_mux as mux;
pub use codetether_native_providers as provider;
pub use codetether_okr as okr;
pub use codetether_platform as platform;
pub use codetether_plugin_marketplace as plugin_marketplace;
pub use codetether_provenance as provenance;
pub mod ralph;
pub mod review;
pub mod rlm;
pub use codetether_native_runtime_policy as runtime_policy;
pub use codetether_native_search as search;
pub use codetether_secrets as secrets;
pub mod server;
pub mod session;
pub mod swarm;
pub mod telemetry;
pub use codetether_tls as tls;
pub mod tool;
pub mod tui;
pub use codetether_native_vectordb as vectordb;
pub use codetether_util::truncate as util;
pub mod voice;
pub mod worker_server;
pub use codetether_util::workspace_scan;
pub use codetether_worktree as worktree;
pub use codetether_worktree_isolation as worktree_stub;
