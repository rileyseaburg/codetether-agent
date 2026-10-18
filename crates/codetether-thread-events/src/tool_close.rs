//! Open tool tracking helpers.

use super::mapper::ThreadEventMapper;

impl ThreadEventMapper {
    pub(super) fn close_open_tool(&mut self, tool_call_id: &str) {
        if let Some(idx) = self
            .open_tools
            .iter()
            .position(|tool| tool.id == tool_call_id)
        {
            self.open_tools.remove(idx);
        }
    }
}
