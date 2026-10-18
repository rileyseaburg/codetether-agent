//! Stable tool item id allocation.

use super::ids::item_id;
use super::mapper::{OpenTool, ThreadEventMapper};

impl ThreadEventMapper {
    pub(super) fn open_tool_item(&mut self, tool_call_id: &str) -> String {
        if let Some(tool) = self.open_tools.iter().find(|tool| tool.id == tool_call_id) {
            return tool.item_id.clone();
        }
        self.item_seq += 1;
        let id = item_id(&self.context.turn_id, self.item_seq);
        self.open_tools.push(OpenTool {
            id: tool_call_id.to_string(),
            item_id: id.clone(),
        });
        id
    }
}
