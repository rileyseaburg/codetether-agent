//! marketing for portable theme configuration.

use crate::{ColorDef, Theme};

impl Theme {
    /// Construct the marketing preset.
    pub fn marketing() -> Self {
        Self {
            // Cyan accent for user (matches marketing site buttons/highlights)
            user_color: ColorDef::Rgb(6, 182, 212), // cyan-400
            // Soft cyan for assistant
            assistant_color: ColorDef::Rgb(34, 211, 238), // cyan-300
            // Yellow for system (warnings/notifications)
            system_color: ColorDef::Rgb(250, 204, 21), // yellow-400
            // Magenta for tools
            tool_color: ColorDef::Rgb(232, 121, 249), // fuchsia-400
            // Red for errors
            error_color: ColorDef::Rgb(248, 113, 113), // red-400
            // Gray-800 for borders (subtle, matches marketing cards)
            border_color: ColorDef::Rgb(31, 41, 55), // gray-800
            // Gray-700 for input borders
            input_border_color: ColorDef::Rgb(55, 65, 81), // gray-700
            // Cyan for help borders
            help_border_color: ColorDef::Rgb(6, 182, 212), // cyan-400
            // Gray-500 for timestamps
            timestamp_color: ColorDef::Rgb(107, 114, 128), // gray-500
            // Gray-600 for code blocks
            code_block_color: ColorDef::Rgb(75, 85, 99), // gray-600
            // Dark foreground for status bar
            status_bar_foreground: ColorDef::Rgb(17, 24, 39), // gray-900
            // Cyan background for status bar
            status_bar_background: ColorDef::Rgb(6, 182, 212), // cyan-400
            // Gray-950 background (near black, matches marketing site)
            background: Some(ColorDef::Rgb(3, 7, 18)), // gray-950
        }
    }
}
