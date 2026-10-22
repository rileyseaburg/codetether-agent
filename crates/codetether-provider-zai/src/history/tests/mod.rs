//! Regression expectations for both existing history conventions.
use super::{HistoryFormat, convert};
use crate::{ContentPart, Message, Role};
use serde_json::json;
mod annotated_tool;
mod packed_replies;
mod separators;
