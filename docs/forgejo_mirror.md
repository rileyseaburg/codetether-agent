# Forgejo is authoritative; GitHub is a code mirror

Develop, review, and release at
<https://forgejo.quantum-forge.io/riley/codetether-agent>.
GitHub receives branches and tags from Forgejo, not the other way around.
Do not make independent commits or releases on GitHub.

## One release pipeline

Only `.forgejo/workflows/release.yml` builds and publishes releases.
GitHub Actions is intentionally disabled in the GitHub repository's settings.
Keep it disabled: mirrored refs must not start the legacy `.github/workflows`
build, package, or release jobs. Existing GitHub releases are historical;
code mirroring does not copy release objects or binary assets.

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

`install.sh` (Linux/macOS) and `install.ps1` (Windows) download Forgejo releases,
including published prereleases, and check the release's SHA256 manifest.
Windows bootstrap helpers are pinned to a reviewed commit and Git-blob verified.
When changing helpers, commit them first and update the bootstrap pin in a
following commit. This allows current helpers to install older binary releases.