//! Interactive terminal setup, teardown, panic recovery and shutdown signals.
//!
//! Use [`enter`] to acquire a [`Runtime`], retaining it for the UI lifetime.
//! [`begin_render_recovery`] temporarily suppresses teardown for render panics
//! that the caller catches. [`spawn_shutdown_listener`] reports process signals.
mod clear;
mod display_mode;
mod enter;
mod frontend;
mod panic_cleanup;
mod recovery_scope;
mod restore;
mod runtime;
mod signals;
mod terminal_guard;
pub use display_mode::{enter as enter_display_mode, leave as leave_display_mode};
pub use enter::enter;
pub use panic_cleanup::{PanicHookGuard, install_panic_cleanup_hook};
pub use recovery_scope::{Guard as RenderRecoveryGuard, enter as begin_render_recovery};
pub use restore::restore_terminal_state;
pub use runtime::Runtime;
pub use signals::spawn_shutdown_listener;
pub use terminal_guard::TerminalGuard;

mod draw;
mod draw_log;
mod draw_recover;
mod draw_resync;
mod draw_size;
pub use draw::draw_ui;
pub use draw_size::safe_size;
