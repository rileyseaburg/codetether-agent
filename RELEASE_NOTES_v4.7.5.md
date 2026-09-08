# CodeTether Agent v4.7.5

## Highlights

- **Patch verification no longer repeatedly cold-starts TypeScript.** A slow
  initial diagnostic check can finish in the background without losing its
  language server. Subsequent edits reuse the warmed server. Explicit
  TypeScript diagnostic requests also handle consecutive clean edits without
  waiting for notifications that the server may suppress.
- **Verification failures are visible.** Patch results distinguish successful
  file writes from unavailable code verification. Automatic checks share an
  interactive budget, concurrent checks avoid duplicate work, and genuinely
  failed servers are evicted with retry backoff.
- **Windows desktop tooling and packaging.** Native desktop interaction, OCR,
  foreground-window safeguards, and shadow-input handling are accompanied by
  a setup bundle and MSI packaging.
- **More reliable sessions and collaboration.** Updates include managed
  worktree isolation, durable bus writes, image-content preservation across
  tools and agent transports, and session transport improvements.
- **Vault credential lifetime management.** Running agents renew eligible
  Vault token leases rather than relying on a token's initial lifetime.
- **Provider and sandbox improvements.** This release includes provider
  discovery and streaming fixes, reasoning controls, and macOS sandbox work.

## Binary packages

- Linux x86-64: GNU/Linux tarball.
- Windows x86-64: GNU executable, setup ZIP, and MSI installer.
- macOS: separate Apple Silicon and Intel tarballs.
- `SHA256SUMS-v4.7.5.txt` covers the published binary packages.

macOS binaries from the Forgejo release workflow are **unsigned and not
notarized**. Windows binaries are cross-built; packaging does not imply a
native Windows runtime smoke test.

## Upgrade notes

Restart existing agent/TUI sessions after upgrading to load the new runtime.
The first diagnostic check in a large project can still exhaust its short
interactive wait; it now keeps warming the server instead of restarting it.