//! Request admission for message/send and message/stream.
//!
//! [send] and [stream] share application hooks without conflating their task
//! state or blocking semantics. [MessageAdmissionHost] supplies execution.
use crate::intro::detect::is_intro;
use crate::task_state as parse;
use crate::task_state::stream::{open, text};
use codetether_a2a_core::*;
use dashmap::DashMap;
mod host;
mod send;
mod stream;
pub use host::MessageAdmissionHost;
pub use send::send;
pub use stream::stream;

mod intro;
