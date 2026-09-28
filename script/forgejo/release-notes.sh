#!/usr/bin/env bash
# Print per-platform install notes for release v$1 (Markdown, stdout).
# Used by .forgejo/workflows/release-publish.yml; the GitHub mirror copies
# the Forgejo release body, so both hosts show the same instructions.
set -euo pipefail
version="${1:?usage: release-notes.sh <version-without-v>}"
tag="v$version"
raw="https://raw.githubusercontent.com/rileyseaburg/codetether-agent/main"
dl="https://github.com/rileyseaburg/codetether-agent/releases/download/$tag"
cat <<EOF
CI-built Linux, Windows, and unsigned macOS binaries with SHA-256 checksums.

## Install

### macOS (Apple Silicon and Intel)

Open Terminal, copy and paste:

\`\`\`sh
curl -fsSL $raw/install.sh | sh
codetether --version
\`\`\`

The installer picks \`aarch64-apple-darwin\` or \`x86_64-apple-darwin\` for your Mac
and verifies it against \`SHA256SUMS-$tag.txt\`. The binary is unsigned; if macOS
blocks it, run \`xattr -d com.apple.quarantine "\$(command -v codetether)"\`.

### Linux (x86_64)

Copy and paste into a terminal:

\`\`\`sh
curl -fsSL $raw/install.sh | sh
codetether --version
\`\`\`

### Windows (x86_64)

Open PowerShell normally (not as Administrator), copy and paste:

\`\`\`powershell
& ([scriptblock]::Create((irm $raw/install.ps1))) -Version $tag
codetether --version
\`\`\`

Or download \`codetether-$tag-x86_64-pc-windows-gnu.msi\` below and double-click it.

### Manual download

Pick your file from the assets below and check it before use:

\`\`\`sh
curl -fsSLO $dl/SHA256SUMS-$tag.txt
sha256sum --check --ignore-missing SHA256SUMS-$tag.txt   # macOS: shasum -a 256 -c
\`\`\`

The one-line macOS/Linux installer installs the newest release; while this
release is the newest it installs exactly $tag.
EOF
