//! Serializable theme fields.

use crate::ColorDef;
use serde::{Deserialize, Serialize};

/// Theme configuration for the TUI
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Theme {
    /// User message color
    pub user_color: ColorDef,
    /// Assistant message color
    pub assistant_color: ColorDef,
    /// System message color
    pub system_color: ColorDef,
    /// Tool message color
    pub tool_color: ColorDef,
    /// Error message color
    pub error_color: ColorDef,
    /// Border color for main areas
    pub border_color: ColorDef,
    /// Input border color
    pub input_border_color: ColorDef,
    /// Help border color
    pub help_border_color: ColorDef,
    /// Timestamp color
    pub timestamp_color: ColorDef,
    /// Code block border color
    pub code_block_color: ColorDef,
    /// Status bar colors
    pub status_bar_foreground: ColorDef,
    pub status_bar_background: ColorDef,
    /// Background color (terminal default if None)
    pub background: Option<ColorDef>,
}
