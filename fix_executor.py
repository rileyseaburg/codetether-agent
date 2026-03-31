import sys

content = sys.stdin.read()
content = content.replace("SwarmEvent::SubTaskUpdate {", "SwarmEvent::SubTaskUpdate(crate::tui::app::state::SubTaskUpdateData {")
content = content.replace('    let _ = tx.try_send(SwarmEvent::SubTaskUpdate(', '    let _ = tx.try_send(SwarmEvent::SubTaskUpdate(crate::tui::app::state::SubTaskUpdateData {')

# A bit hacked together but let's try to fix exact spots
sys.stdout.write(content)
