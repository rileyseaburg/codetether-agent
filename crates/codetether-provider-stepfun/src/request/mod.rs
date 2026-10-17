//! Message, image, tool, and request-body conversion.
use crate::*;
mod assistant;
mod build;
mod image_messages;
mod messages;
mod system;
mod text;
mod tool;
mod tools;
mod user;
mod wire;
pub(crate) use wire::ChatRequest;
