//! Buffered retry regression tests.

use super::send_with_retry;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

mod content;
mod exhaustion;
mod network;
mod permanent;
mod recovery;
mod status;
