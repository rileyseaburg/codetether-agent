import re

with open('src/a2a/worker.rs', 'r') as f:
    text = f.read()

# Fix 1: data.status instead of status in SubTaskUpdate(data)
text = text.replace('format!("{:?}", status).to_ascii_lowercase()', 'format!("{:?}", data.status).to_ascii_lowercase()')

# Fix 2: subtask_data.id.unwrap_or_default() -> subtask_id
text = re.sub(r'&subtask_data\.id\.unwrap_or_default\(\)\.chars\(\)\.take\(8\)\.collect::<String>\(\)', '&subtask_id.chars().take(8).collect::<String>()', text)

# Fix 3: removed bindings in Complete
text = text.replace('completed_tasks: _, failed_tasks: _ } => Some(format!(', 'completed_tasks, failed_tasks } => Some(format!(')

with open('src/a2a/worker.rs', 'w') as f:
    f.write(text)
