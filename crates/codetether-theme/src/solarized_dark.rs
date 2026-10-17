//! solarized dark for portable theme configuration.

use crate::{ColorDef, Theme};

impl Theme {
    /// Construct the solarized dark preset.
    pub fn solarized_dark() -> Self {
        Self {
            user_color: ColorDef::Rgb(133, 153, 0),              // green
            assistant_color: ColorDef::Rgb(42, 161, 152),        // cyan
            system_color: ColorDef::Rgb(181, 137, 0),            // yellow
            tool_color: ColorDef::Rgb(211, 54, 130),             // magenta
            error_color: ColorDef::Rgb(220, 50, 47),             // red
            border_color: ColorDef::Rgb(131, 148, 150),          // base1
            input_border_color: ColorDef::Rgb(238, 232, 213),    // base2
            help_border_color: ColorDef::Rgb(181, 137, 0),       // yellow
            timestamp_color: ColorDef::Rgb(101, 123, 131),       // base01
            code_block_color: ColorDef::Rgb(88, 110, 117),       // base03
            status_bar_foreground: ColorDef::Rgb(0, 43, 54),     // base03
            status_bar_background: ColorDef::Rgb(238, 232, 213), // base2
            background: Some(ColorDef::Rgb(0, 43, 54)),          // base03
        }
    }
}
