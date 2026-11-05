//! Bounded MCP response decoding, split by framing and protocol responsibility.
mod frames;
mod protocol;
mod read;
#[cfg(test)]
use frames::frame_end;
pub(crate) use protocol::{event, result};
pub(crate) use read::read;
#[cfg(test)]
#[path = "response_tests.rs"]
mod tests;
