# Build on Forgejo; distribute through GitHub

Develop, review, and release at
<https://forgejo.quantum-forge.io/riley/codetether-agent>.
GitHub receives branches and tags from Forgejo, not the other way around.
Do not make independent code changes or rebuild releases on GitHub.

## One release pipeline

Only `.forgejo/workflows/release.yml` builds the binaries. GitHub releases receive
copies of the same metadata, assets and checksum manifest, not another build.
GitHub Actions stays disabled; publishing assets through its API does not require it.
The README on both sites intentionally uses GitHub install/download URLs because
some user networks cannot access Forgejo. Git push mirrors alone do not copy releases.

The [release-copy service](../script/release-mirror/README.md) checks all
published Forgejo releases every five minutes on the operator host. It requires
matching Git refs and verified checksums, publishes only fully populated GitHub
releases, and refuses same-name asset conflicts instead of overwriting them.

## Push mirror

Forgejo repository settings contain an SSH push mirror to
`ssh://git@github.com/rileyseaburg/codetether-agent.git`.
It syncs on commit, with an eight-hour periodic fallback.
Its generated public key is registered as a write-enabled deploy key on that
GitHub repository; no personal access token is stored for mirroring.
Push mirrors can overwrite/delete destination refs: preserve GitHub-only work
in Forgejo before enabling or repairing one. Check the mirror's last update
and error, then compare branch/tag hashes on both remotes.

## Installers

`install.sh` (Linux/macOS) and `install.ps1` (Windows) download GitHub release mirrors,
including published prereleases, and check the copied SHA256 manifest.
Windows bootstrap helpers are pinned to a reviewed commit and Git-blob verified.
When changing helpers, commit them first and update the bootstrap pin in a
following commit. This allows current helpers to install older binary releases.