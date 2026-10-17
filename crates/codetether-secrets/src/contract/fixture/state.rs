//! Scripted responses and captured requests.
use serde_json::Value;
use std::collections::VecDeque;

#[derive(Default)]
pub struct Calls {
    pub replies: VecDeque<(u16, Value)>,
    pub requests: Vec<(String, String, Value)>,
}
