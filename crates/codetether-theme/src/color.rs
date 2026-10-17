//! Serializable named, RGB, and indexed colors.

use serde::{Deserialize, Serialize};

/// Color definition that can be serialized/deserialized
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ColorDef {
    /// Named color
    Named(String),
    /// RGB color (r, g, b)
    Rgb(u8, u8, u8),
    /// Indexed color (for 256-color terminals)
    Indexed(u8),
}
