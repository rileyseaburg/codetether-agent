//! Streaming retry regression tests.

use super::send_response_with_retry;
use std::sync::atomic::{AtomicU32, Ordering};

mod body;
mod content;
mod network;
mod status;

fn response(status: u16, body: &str) -> reqwest::Response {
    http::Response::builder()
        .status(status)
        .body(body.to_owned())
        .unwrap()
        .into()
}
