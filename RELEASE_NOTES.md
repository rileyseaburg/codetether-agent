# v4.5.0

Release v4.5.0

## Commits
59da816 fix: switch macOS build to password-based SSH (sshpass) and gate on tags only
236ef8a feat: macOS builds on local Mac Mini via Jenkins
2bb1c72 fix: address PR #53 review feedback
d64f519 chore: cargo fix --lib --tests (remove unused imports)
29e8b68 refactor(browserctl): split 542-line mod.rs into SRP modules
fc56235 chore: remove accidental worktree gitlink
92830e7 fix: correct import path for request_git_credentials and remove unnecessary Client instantiation
cb19d42 fix: remove unused imports and delete unlinked runtime_env.rs
4e11827 fix: retry transient LLM network errors (#33)
b18100f fix: address all remaining PR #33 reviewer feedback
b5c6bef fix: address PR #33 reviewer feedback
4cb6fbe fix: remove dead branch_cleanup.rs (not wired into module tree)
1f88be1 fix: delete worktree branches after cleanup to prevent branch leak
bf1e48e refactor: split modules for 50-line limit and SRP compliance
7d6f733 fix: prevent panic on non-char-boundary byte slice in provider error messages
304563c Initial plan
