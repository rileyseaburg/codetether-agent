//! light for portable theme configuration.

use crate::{ColorDef, Theme};

impl Theme {
    /// Construct the light preset.
    pub fn light() -> Self {
        Self {
            user_color: ColorDef::Named("darkgreen".to_string()),
            assistant_color: ColorDef::Named("darkblue".to_string()),
            system_color: ColorDef::Named("darkyellow".to_string()),
            tool_color: ColorDef::Named("darkmagenta".to_string()),
            error_color: ColorDef::Named("darkred".to_string()),
            border_color: ColorDef::Named("darkgray".to_string()),
            input_border_color: ColorDef::Named("black".to_string()),
            help_border_color: ColorDef::Named("darkyellow".to_string()),
            timestamp_color: ColorDef::Named("gray".to_string()),
            code_block_color: ColorDef::Named("gray".to_string()),
            status_bar_foreground: ColorDef::Named("white".to_string()),
            status_bar_background: ColorDef::Named("black".to_string()),
            background: Some(ColorDef::Named("white".to_string())),
        }
    }
}
