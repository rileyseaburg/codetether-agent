import sys
content = sys.stdin.read()
if "use crate::tui::swarm_view::{" in content:
    content = content.replace("use crate::tui::swarm_view::{AgentMessageEntry, AgentToolCallDetail, SubTaskInfo, SwarmEvent};", 
                              "use crate::tui::swarm_view::{AgentMessageEntry, AgentToolCallDetail, SubTaskInfo};\nuse crate::tui::app::state::SwarmEvent;")
sys.stdout.write(content)
