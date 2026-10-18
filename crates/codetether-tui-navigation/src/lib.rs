//! Keyboard navigation priority, scrolling and cursor routing policy.
//!
//! Implement [Host] to supply current [Context] and apply [Action] operations.
//! Handlers preserve overlay priority and delegate state mutation to the host.
mod action;
mod arrows;
mod editing;
mod escape;
mod host;
mod jump;
mod paging;
mod subagent;
pub use action::Action;
pub use arrows::{handle_down, handle_up};
pub use editing::{handle_delete, handle_left, handle_right};
pub use escape::{handle_escape, toggle_help};
pub use host::{Context, Host};
pub use jump::{handle_end, handle_home};
pub use paging::{handle_page_down, handle_page_up};

/// Agent focus ordering, cycling and detail transitions.
pub mod focus;

/// Terminal stream event routing and editor paste priority.
pub mod terminal;

/// Unmodified key dispatch and Enter/Escape priority rules.
pub mod keys;

/// Control/Alt shortcut priority, actions and status policy.
pub mod modified;

/// Mode-specific settings, character and filter input routing.
pub mod modes;

/// Ordered key-event admission and modal dispatch.
pub mod events;

/// View-aware character/backspace routing and input-mode policy.
pub mod text;

/// View-level Enter handler selection.
pub mod enter;

/// Mouse wheel priorities, view scrolling and click routing.
pub mod mouse;
