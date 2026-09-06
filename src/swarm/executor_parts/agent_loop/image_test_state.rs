//! Construct isolated swarm state with offline image fixtures.

use super::{
    create, image_test_provider::OfflineProvider, image_test_tool::ImageTool, state::State,
};
use crate::tool::{ToolRegistry, ToolResult};
use std::sync::Arc;

pub(super) fn state(result: ToolResult) -> State {
    let mut registry = ToolRegistry::new();
    registry.register(Arc::new(ImageTool(result)));
    create::state(
        Arc::new(OfflineProvider),
        "offline",
        "system",
        "user",
        Vec::new(),
        Arc::new(registry),
        1,
        30,
        None,
        "image-test".into(),
        None,
        None,
    )
}
