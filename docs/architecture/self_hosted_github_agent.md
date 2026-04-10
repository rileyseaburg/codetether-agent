# Self-Hosted GitHub Agent Architecture

## Goal

Run CodeTether as the actual agent runtime for GitHub issues and pull requests.
GitHub should provide identity, webhooks, repository access, comments, branches,
pull requests, and status surfaces, but should not host the agent session or bill
the run as a Copilot premium request.

## Problem With The Current Path

The current `.github/agents/codetether.agent.md` path creates a GitHub Copilot
custom agent. That keeps GitHub Copilot as the outer runtime, model host, and
billing boundary. The visible symptoms are:

- session actor is `Copilot`
- model is selected by GitHub cloud agent
- usage is counted as a GitHub premium request
- `codetether` appears only as an inner custom agent profile

That path is useful for integration testing, but it does not satisfy the
requirement that CodeTether itself be the cloud agent.

## Non-Negotiable Constraints

- CodeTether must execute on our infrastructure
- GitHub App identity must be the external actor for comments, branches, and PRs
- GitHub premium requests must not be consumed for CodeTether runs
- Runs must be resumable and observable outside GitHub
- GitHub must remain the user-facing source for issue and PR collaboration

## Platform Reality

GitHub's native AI assignee UI for `Copilot`, `Claude`, and `Codex` is a GitHub
platform capability. A normal GitHub App cannot replace that surface purely with
repository files or workflows. We can mimic the behavior closely, but not occupy
the same first-party Copilot slot unless GitHub offers CodeTether a partner-grade
third-party coding-agent integration.

## Proposed Solution

Build a first-party CodeTether GitHub App workflow with five components:

1. GitHub App
   - installation auth
   - webhook source
   - comment/PR/branch/check-run writer
2. Webhook ingress
   - validates GitHub signatures
   - normalizes issue, PR, review, and comment events into CodeTether tasks
3. Orchestrator
   - decides whether work is review, implementation, follow-up, or resume
   - allocates workspace and model
   - tracks session state
4. Runner
   - runs `codetether run` or Ralph on our infrastructure
   - owns branch creation, commits, test execution, and patch application
5. Session reporter
   - posts issue comments
   - updates PR description and draft state
   - creates check runs and progress summaries

## User Experience To Mimic Copilot

From GitHub's point of view:

- user mentions `@codetether` on an issue or PR
- or labels an issue `codetether`
- or clicks a GitHub App action entry point
- CodeTether immediately posts a "session started" comment
- CodeTether creates a branch and draft PR when implementation starts
- CodeTether updates a check run named `codetether/agent`
- CodeTether posts progress checkpoints and a final summary
- user requests follow-up by commenting on the PR

This preserves the GitHub-native feel without depending on GitHub's cloud agent.

## Webhook Events

The minimum event set is:

- `issues`
- `issue_comment`
- `pull_request`
- `pull_request_review_comment`
- `pull_request_review`
- `check_run`
- `installation`
- `installation_repositories`

## GitHub App Permissions

The minimum repository permissions are:

- `Contents`: read/write
- `Pull requests`: read/write
- `Issues`: read/write
- `Metadata`: read-only
- `Checks`: read/write
- `Commit statuses`: read/write

Optional permissions:

- `Actions`: read-only if we want workflow awareness
- `Discussions`: read/write if we extend to discussions

## Request Routing

Map GitHub events into CodeTether tasks:

- issue labeled or mentioned:
  - create `issue.execute`
- PR opened or synchronized:
  - create `pr.review`
- PR comment asking for changes:
  - create `pr.fix`
- issue comment on an active session:
  - create `session.steer`
- PR merge or issue close:
  - create `session.archive`

Each task stores:

- installation id
- repository id and full name
- issue or PR number
- head and base refs
- requested mode (`review`, `fix`, `implement`, `resume`)
- session id

## Execution Model

Every run should:

1. create or resume a CodeTether session id
2. clone a clean worktree for the target repository
3. fetch refs and checkout the correct base/head
4. run CodeTether with task-specific prompt scaffolding
5. capture structured progress, tool usage, logs, and artifacts
6. write code, run validation, and commit changes
7. push a branch if implementation work was requested
8. create or update a PR
9. publish final status back to GitHub

## GitHub Surfaces

Use four GitHub surfaces to approximate Copilot:

- issue comments for conversational updates
- draft PRs for implementation output
- check runs for machine-readable progress
- branch naming convention like `codetether/issue-16-byte-index-crash`

Check run phases:

- `queued`
- `in_progress`
- `needs_input`
- `failed`
- `completed`

## Session And State Model

The orchestrator should persist:

- GitHub installation and repo metadata
- CodeTether session id
- active branch name
- linked PR number
- last webhook event id
- state transition timestamps
- retry count and failure reason

Recommended states:

- `queued`
- `planning`
- `implementing`
- `validating`
- `waiting_for_user`
- `completed`
- `failed`
- `canceled`

## Security Model

- validate GitHub webhook HMAC signature
- exchange installation id for short-lived installation token
- never persist installation tokens beyond the session
- scope repository access to the single target installation
- isolate each run in a fresh worktree or container
- require explicit policy checks before destructive git actions

## Components In This Ecosystem

Recommended ownership split:

- `a2a_server`
  - webhook ingestion
  - task orchestration
  - session registry
  - GitHub status publishing
- `codetether-agent`
  - task execution
  - code editing
  - tests and validation
  - branch and commit operations

## Immediate Build Plan

Phase 1:

- add GitHub App webhook handler in `a2a_server`
- translate mention/label/review events into dispatch tasks
- add check-run publishing

Phase 2:

- add `github app` auth mode to CodeTether runner
- add `issue.execute` and `pr.fix` runner prompts
- add branch and PR creation helpers

Phase 3:

- add session dashboard in CodeTether UI
- add resume, steer, and cancel operations
- add issue-to-PR closure automation

## Success Criteria

The solution is complete when:

- assigning or invoking CodeTether does not consume GitHub premium requests
- the active model and logs are controlled by our infrastructure
- CodeTether can open and update PRs directly as the GitHub App
- progress is visible from GitHub without using Copilot cloud sessions
- issue follow-up comments resume the same CodeTether session
