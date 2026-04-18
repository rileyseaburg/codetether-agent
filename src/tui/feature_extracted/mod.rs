//! Partially-integrated extracted modules.
//!
//! Remaining files contain code not yet ported due to deep cross-module
//! dependencies or pending architecture stabilization:
//! - autochat_relay.rs: relay worker orchestration (needs OKR/Ralph/Swarm)
//! - relay_planning.rs: relay profile generation (needs OKR/Ralph)
//! - watchdog_and_pickers.rs: old-shape impl App methods (needs event_loop refactor)
//! - webview.rs: inspector + protocol rendering (old shape)

#[allow(dead_code)]
pub mod autochat_relay;
#[allow(dead_code)]
pub mod relay_planning;
#[allow(dead_code)]
pub mod watchdog_and_pickers;
#[allow(dead_code)]
pub mod webview;
