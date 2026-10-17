//! dark for portable theme configuration.

use crate::{ColorDef, Theme};

impl Theme {
    /// Construct the dark preset.
    pub fn dark() -> Self {
        Self {
            user_color: ColorDef::Named("green".to_string()),
            assistant_color: ColorDef::Named("cyan".to_string()),
            system_color: ColorDef::Named("yellow".to_string()),
            tool_color: ColorDef::Named("magenta".to_string()),
            error_color: ColorDef::Named("red".to_string()),
            border_color: ColorDef::Named("cyan".to_string()),
            input_border_color: ColorDef::Named("white".to_string()),
            help_border_color: ColorDef::Named("yellow".to_string()),
            timestamp_color: ColorDef::Named("darkgray".to_string()),
            code_block_color: ColorDef::Named("darkgray".to_string()),
            status_bar_foreground: ColorDef::Named("black".to_string()),
            status_bar_background: ColorDef::Named("white".to_string()),
            background: None,
        }
    }
}
