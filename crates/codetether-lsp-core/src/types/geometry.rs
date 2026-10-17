//! LSP geometry definitions and helpers.

use super::*;
use lsp_types::{Location, Position, Range};

/// Simplified location info for tool output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationInfo {
    pub uri: String,
    pub range: RangeInfo,
}

impl From<Location> for LocationInfo {
    fn from(loc: Location) -> Self {
        Self {
            uri: loc.uri.to_string(),
            range: RangeInfo::from(loc.range),
        }
    }
}

/// Simplified range info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RangeInfo {
    pub start: PositionInfo,
    pub end: PositionInfo,
}

impl From<Range> for RangeInfo {
    fn from(range: Range) -> Self {
        Self {
            start: PositionInfo::from(range.start),
            end: PositionInfo::from(range.end),
        }
    }
}

/// Simplified position info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionInfo {
    pub line: u32,
    pub character: u32,
}

impl From<Position> for PositionInfo {
    fn from(pos: Position) -> Self {
        Self {
            line: pos.line,
            character: pos.character,
        }
    }
}
