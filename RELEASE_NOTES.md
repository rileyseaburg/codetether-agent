# v4.0.1

Release v4.0.1

## Commits
f775e5e chore: update repository changes
6550b21 fix: simplify tool lookup logic in list_tools_bootstrap_output
a5fb30c fix: stabilize local qwen kv cache reuse and reset
95eacaa chore: ship local-cuda stability and model default updates
162e537 Update Cargo.toml
ecf5dce chore: bump version to 4.0.1
fac3466 chore: update repository changes
c49dc87 chore: update repository changes
e30b55d chore: update Cargo.lock
cc8d4b8 fix: generate Cargo.lock before Docker build and fix context path
059ff48 fix: correct Dockerfile context path in CI workflow
070234a docs: update README with v4.0.1 changes
0985a68 fix: add serde(default) to retry_count for backward compatibility
764d1aa docs: update README with v4.0.1 changes
0b1131d fix: resolve merge conflicts and compilation errors
b2dfea9 feat: update swarm executor and remote subtask handling
bc901a0 fix: update swarm worktree task branch
e3d1b4e chore: update chat events and session data
0f81eea fix: update commit.sh to push to current branch dynamically
cef87a7 chore: update chat events and session data
fba8943 fix: update commit.sh to push to current branch dynamically
d4dfb1e chore: update repository changes
a5f37b1 feat: make morph primary backend for edit and multiedit
a7c3cb4 fix: capture swarm retry metadata and worktree pointers
a269c8e fix: update swarm worktree task branch
be03491 fix: update swarm worktree task branch
4a56cec chore: snapshot remaining workspace changes
77f0d44 feat: add moonshot rubric for forage and workspace data defaults
53be842 chore: update repository changes
d47437a feat: add shared relay context, model rotation, cookie auth, and git integrity checks for autochat relay
e8f7cb0 feat: enforce PRD-first autochat with tactical opt-out
5276873 feat: apply relay/autochat refactor and pending workspace updates
b2cf101 feat(provider): enhance GLM-5 FP8 provider with model specs and recommended list
4947652 fix: suppress never_loop clippy false positive in session retry loop
b2a3b0d fix: enforce explicit provider routing and bootstrap local mode
00f3adc fix: prevent explicit local provider fallback to remote
29667af feat: normalize relay handoffs across tui autochat
e712f86 feat: enforce grep evidence and normalize FINAL payloads
4c2804e feat: enforce FINAL JSON output and harden local CUDA RLM
65f40d1 feat: realign oracle-first rlm flow and persistence
ab2457d fix: address PR review feedback for RLM, worktree, and S3 tests
1b768bb feat: wire oracle verification and consensus into rlm workflow
d8e56c8 feat: implement git worktree operations, S3 backwards compatibility, and security hardening
