//! Shared TUI model types.
//!
//! This module defines small serializable value types used across the terminal
//! interface. It includes the set of available views, the current keyboard input
//! mode, color theme settings, and workspace snapshot re-exports used by view
//! state and rendering modules.

/// Display formatting helpers for [`ViewMode`].
pub mod view_mode_display;
/// Human-readable help text for [`ViewMode`] values.
pub mod view_mode_help;
/// Registry utilities for enumerating and selecting [`ViewMode`] values.
pub mod view_mode_registry;

use serde::{Deserialize, Serialize};

/// Top-level screens available in the terminal UI.
///
/// `ViewMode` records which major panel is currently active. It is serialized
/// with session or UI state so the interface can preserve and restore the
/// selected screen when appropriate.
///
/// Variants correspond to user-facing views such as chat, saved sessions,
/// swarm execution, Ralph automation, bus logs, model selection, settings,
/// language tooling, latency metrics, protocol inspection, file picking,
/// audit logs, and git status.
///
/// # Examples
///
/// ```
/// use codetether_agent::tui::models::ViewMode;
///
/// let mode = ViewMode::Chat;
/// assert_eq!(mode, ViewMode::Chat);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ViewMode {
    /// Main conversation view for prompts, assistant responses, and tool output.
    Chat,
    /// Saved-session browser and session switching view.
    Sessions,
    /// Parallel swarm task execution and result view.
    Swarm,
    /// Ralph PRD-driven autonomous development view.
    Ralph,
    /// Agent bus event and collaboration log view.
    Bus,
    /// Model picker and provider model selection view.
    Model,
    /// User-configurable TUI and runtime settings view.
    Settings,
    /// Language Server Protocol diagnostics and symbol tooling view.
    Lsp,
    /// Recursive Language Model processing view.
    Rlm,
    /// Request, tool, and streaming latency metrics view.
    Latency,
    /// Transport-health view: TCP_INFO probe (RTT, cwnd, retransmits).
    Transport,
    /// Protocol-level message inspection view.
    Protocol,
    /// Workspace file picker view.
    FilePicker,
    /// Inspector view for detailed application or message state.
    Inspector,
    /// Audit trail view for security and operational events.
    Audit,
    /// Git status and repository change view.
    Git,
    /// Audit-loop view: implement → audit → retry cycle visualization.
    AuditLoop,
    /// In-TUI text editor view for editing workspace files.
    Editor,
}

/// Keyboard input mode currently used by the TUI.
///
/// The input mode determines how key presses are interpreted: as navigation,
/// text editing, or command entry. It is separate from [`ViewMode`] so every
/// screen can share consistent keyboard behavior.
///
/// # Examples
///
/// ```
/// use codetether_agent::tui::models::InputMode;
///
/// let mode = InputMode::Normal;
/// assert_eq!(mode, InputMode::Normal);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InputMode {
    /// Navigation mode where keys control focus, scrolling, and view actions.
    Normal,
    /// Text-entry mode for composing or editing prompt input.
    Editing,
    /// Slash-command or command-line mode.
    Command,
}

/// Color theme configuration for TUI rendering.
///
/// The theme stores a display name and RGB tuples for primary accents and
/// borders. Rendering code can convert these tuples into terminal colors while
/// keeping persisted configuration simple and serializable.
///
/// # Invariants
///
/// Each color channel is an `u8`, so values are always valid RGB channel values
/// in the range `0..=255`.
///
/// # Examples
///
/// ```
/// use codetether_agent::tui::models::Theme;
///
/// let theme = Theme::default();
/// assert_eq!(theme.name, "Default");
/// assert_eq!(theme.primary, (0, 255, 255));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    /// Human-readable theme name shown in settings or diagnostics.
    pub name: String,
    /// Primary accent color as an RGB tuple.
    pub primary: (u8, u8, u8),
    /// Border color as an RGB tuple.
    pub border: (u8, u8, u8),
}

impl Default for Theme {
    /// Creates the default cyan-accented TUI theme.
    ///
    /// # Returns
    ///
    /// Returns a [`Theme`] named `"Default"` with a cyan primary color and a
    /// neutral gray border color.
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            primary: (0, 255, 255),
            border: (100, 100, 100),
        }
    }
}

/// Workspace item and snapshot types shared by file picker and workspace views.
pub use crate::tui::utils::workspace::{WorkspaceEntry, WorkspaceEntryKind, WorkspaceSnapshot};
