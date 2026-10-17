# Go Pipeline Launched

**OKR ID:** `{okr_id_str}`
**Task:** {task}
**Model:** {resolved_model}
**Max Iterations:** {max_iterations}
**Working Directory:** {working_dir}

The autonomous pipeline is now running in the background.

## Monitor Progress

Use the `go` tool with action `watch` to monitor this pipeline:

```json
{{"action": "watch", "okr_id": "{okr_id_str}"}}
```

The pipeline will:
1. Generate a PRD from the task description
2. Run the Ralph loop to implement all user stories
3. Run quality checks (typecheck, lint, test, build)
4. Map results back to OKR outcomes

PRD file: `{prd_filename}`
Progress file: `{progress_filename}`