# CodeTether Agent Identity

This worktree is configured to commit as the autonomous agent rather than
the human operator. All commits and tags produced here are SSH-signed by
the agent's dedicated signing key.

| Field | Value |
|------|------|
| `user.name` | `CodeTether Agent` |
| `user.email` | `agent+codetether@codetether.run` |
| `gpg.format` | `ssh` |
| `user.signingkey` | `~/.ssh/codetether_agent_signing.pub` |
| `gpg.ssh.allowedSignersFile` | `~/.ssh/codetether_allowed_signers` |
| `commit.gpgsign` | `true` |
| `tag.gpgsign` | `true` |

The email follows the scheme used by
[`src/provenance/identity.rs`](../../src/provenance/identity.rs)
(`agent+<identity>@codetether.run`), so commits made here are consistent
with commits produced by the agent's runtime provenance system.

Verify any commit with:

```
git -c gpg.ssh.allowedSignersFile=~/.ssh/codetether_allowed_signers \
    log --show-signature -1
```
