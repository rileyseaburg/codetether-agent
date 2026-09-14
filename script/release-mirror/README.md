# Forgejo → GitHub release copy service

Builds stay in Forgejo. This copies public metadata, platform files and the SHA256
manifest to GitHub using the operator's already-provisioned `gh` authentication.
It does not copy credentials into Forgejo CI, enable GitHub Actions, or build binaries.

```sh
# Reconcile all published releases (or supply an explicit tag).
node script/release-mirror/run.mjs
node script/release-mirror/run.mjs v4.7.6-dev.6
```

Requires Node with built-in fetch, Git, and authenticated GitHub CLI access to
`rileyseaburg/codetether-agent`. The code/tag push mirror must already contain the
source commit. Draft source releases and incomplete source asset sets are refused.
Missing GitHub releases are assembled as drafts and published only after all
source assets match their checksums. Existing matching assets are skipped;
same-name digest conflicts are refused, never overwritten or deleted.

## Persistent operator service

```sh
bash script/release-mirror/install-user-timer.sh
systemctl --user status codetether-release-mirror.timer
```

The timer checks every five minutes after its previous run. User lingering is
required to run after logout. Evidence is retained under
`~/.local/state/codetether-release-mirror`; GitHub CLI credentials remain in their
existing store. Inspect failures with `journalctl --user -u codetether-release-mirror.service`.