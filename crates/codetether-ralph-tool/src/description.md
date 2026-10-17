Run the Ralph autonomous agent loop to implement user stories from a PRD.

Ralph is an autonomous AI agent loop that runs repeatedly until all PRD items are complete.
Each iteration is a fresh instance with clean context. Memory persists via:
- Git history (commits from previous iterations)
- progress.txt (learnings and context)
- prd.json (which stories are done)

After completion, Ralph:
- Cleans up orphaned worktrees and branches
- Returns to your original branch
- Provides next steps (merge instructions or retry guidance)

The calling agent should handle the final merge based on the result metadata.

Actions:
- run: Start the Ralph loop with a PRD file
- status: Check progress of current Ralph run
- create-prd: Create a new PRD template

Returns metadata: {all_passed, ready_to_merge, feature_branch, passed, total}
