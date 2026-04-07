//! Feature-extracted modules from codetether/tui-codetether-agent branch
//!
//! These modules contain features that exist in the branch's 12K-line
//! `src/tui/mod.rs` monolith but have not yet been integrated into
//! main's modular TUI structure.
//!
//! ## Integration status
//!
//! Each module is self-contained. To integrate:
//! 1. Review imports and dependencies
//! 2. Add types/functions to the appropriate main TUI module
//! 3. Wire into event loop or render pipeline
//! 4. Remove from this directory once integrated

pub mod agent_identity;
pub mod autochat_relay;
pub mod chat_sync;
pub mod clipboard;
pub mod commands_ext;
pub mod file_picker_ext;
pub mod relay_planning;
pub mod smart_switch_ext;
pub mod view_modes;
pub mod watchdog_and_pickers;
pub mod webview;
