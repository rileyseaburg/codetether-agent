//! color named for portable theme configuration.

use crate::ColorDef;

impl ColorDef {
    /// Construct the black named color.
    pub fn black() -> Self {
        ColorDef::Named("black".to_string())
    }

    /// Construct the red named color.
    pub fn red() -> Self {
        ColorDef::Named("red".to_string())
    }

    /// Construct the green named color.
    pub fn green() -> Self {
        ColorDef::Named("green".to_string())
    }

    /// Construct the yellow named color.
    pub fn yellow() -> Self {
        ColorDef::Named("yellow".to_string())
    }

    /// Construct the blue named color.
    pub fn blue() -> Self {
        ColorDef::Named("blue".to_string())
    }

    /// Construct the magenta named color.
    pub fn magenta() -> Self {
        ColorDef::Named("magenta".to_string())
    }

    /// Construct the cyan named color.
    pub fn cyan() -> Self {
        ColorDef::Named("cyan".to_string())
    }

    /// Construct the white named color.
    pub fn white() -> Self {
        ColorDef::Named("white".to_string())
    }

    /// Construct the gray named color.
    pub fn gray() -> Self {
        ColorDef::Named("gray".to_string())
    }

    /// Construct the dark gray named color.
    pub fn dark_gray() -> Self {
        ColorDef::Named("darkgray".to_string())
    }

    /// Construct the light gray named color.
    pub fn light_gray() -> Self {
        ColorDef::Named("gray".to_string())
    }
}
