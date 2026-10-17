//! solarized light for portable theme configuration.

use crate::{ColorDef, Theme};

impl Theme {
    /// Construct the solarized light preset.
    pub fn solarized_light() -> Self {
        Self {
            user_color: ColorDef::Rgb(133, 153, 0),              // green
            assistant_color: ColorDef::Rgb(42, 161, 152),        // cyan
            system_color: ColorDef::Rgb(181, 137, 0),            // yellow
            tool_color: ColorDef::Rgb(211, 54, 130),             // magenta
            error_color: ColorDef::Rgb(220, 50, 47),             // red
            border_color: ColorDef::Rgb(88, 110, 117),           // base00
            input_border_color: ColorDef::Rgb(147, 161, 161),    // base1
            help_border_color: ColorDef::Rgb(181, 137, 0),       // yellow
            timestamp_color: ColorDef::Rgb(131, 148, 150),       // base1
            code_block_color: ColorDef::Rgb(147, 161, 161),      // base1
            status_bar_foreground: ColorDef::Rgb(238, 232, 213), // base2
            status_bar_background: ColorDef::Rgb(88, 110, 117),  // base00
            background: Some(ColorDef::Rgb(253, 246, 227)),      // base3
        }
    }
}
