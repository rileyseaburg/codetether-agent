//! Regression expectations for both existing history conventions.
use super::{HistoryFormat, convert};
use crate::{ContentPart, Message, Role};
use serde_json::json;
mod separators;
mod packed_replies;
mod annotated_tool;
