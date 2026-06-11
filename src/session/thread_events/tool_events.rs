//! Multi-event tool lifecycle mapping.

use crate::session::thread_store::ThreadEvent;

use super::mapper::ThreadEventMapper;

impl ThreadEventMapper {
    pub(super) fn tool_start_events(
        &mut self,
        tool_call_id: &str,
        name: &str,
        arguments: &str,
    ) -> Vec<ThreadEvent> {
        let mut events = vec![self.tool_started(tool_call_id, name, arguments)];
        if name == "bash" {
            events.push(self.command_started(tool_call_id, arguments));
        } else if name == "apply_patch" {
            events.push(self.patch_started(tool_call_id, arguments));
        }
        events
    }

    pub(super) fn tool_complete_events(
        &mut self,
        tool_call_id: &str,
        name: &str,
        output: &str,
        success: bool,
        duration_ms: u64,
    ) -> Vec<ThreadEvent> {
        let done = self.tool_completed(tool_call_id, name, output, success, duration_ms);
        let mut events = vec![done];
        if name == "bash" {
            events.push(self.command_completed(tool_call_id, success, duration_ms));
        }
        self.close_open_tool(tool_call_id);
        events
    }
}
