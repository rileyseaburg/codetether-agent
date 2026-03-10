import sys

content = sys.stdin.read()

# Fix imports in worker first:
content = content.replace("crate::tui::swarm_view::SwarmEvent", "crate::tui::app::state::SwarmEvent")

def patch_match(text):
    text = text.replace(
        "SwarmEvent::Started {\n            task,\n            total_subtasks,\n        }",
        "SwarmEvent::Started {\n            task,\n            total_subtasks,\n        }"
    )
    text = text.replace(
        "SwarmEvent::StageComplete {\n            stage,\n            completed,\n            failed,\n        }",
        "SwarmEvent::StageComplete {\n            stage,\n            completed,\n            failed,\n        }"
    )
    # The SubTaskUpdate has been changed to SubTaskUpdate(data)
    text = text.replace(
        "SwarmEvent::SubTaskUpdate { id, status, .. }",
        "SwarmEvent::SubTaskUpdate(data)"
    )
    text = text.replace(
        "id.chars()",
        "data.id.unwrap_or_default().chars()"
    )
    text = text.replace(
        "status",
        "data.status"
    )

    # Tool call
    text = text.replace(
        "SwarmEvent::AgentToolCall {\n            subtask_id,\n            tool_name,\n            ..",
        "SwarmEvent::AgentToolCall {\n            subtask_id,\n            tool_name,\n            .."
    )
    # It changed to subtask_id: Option<String>, tool_name: Option<String> etc.
    text = text.replace(
        "subtask_id.chars()",
        "subtask_id.unwrap_or_default().chars()"
    )
    
    text = text.replace("success, stats", "success, final_summary: _, total_tasks: _, completed_tasks: _, failed_tasks: _")
    text = text.replace("stats.subagents_completed + stats.subagents_failed", "completed_tasks + failed_tasks")
    text = text.replace("stats.speedup_factor", "0.0")

    return text

sys.stdout.write(patch_match(content))
