# Paired-session follow-up: Windows Ethernet 6

## Scope and provenance

The user confirmed pairing and corrected the active Windows address to
`riley@192.168.50.135` (Ethernet 6). Historical `.187` connection failures do
not describe current reachability. No session was stopped or replaced.

## Observed this follow-up — static/local host and file inspection

- Key-only SSH `hostname && whoami` exited 0: `rileys-laptop` and
  `rileys-laptop\riley`.
- `Get-Process codetether-companion` returned PID 1124, session 1, at
  `C:\Users\riley\AppData\Local\Programs\CodeTetherCompanion\codetether-companion.exe`.
- Installed SHA-256:
  `881C13E57AD09667ECC3687075A549DFF244A791ED5CABAC3F26FE347EEA3FE0`.
  This matches the retained `../client-deploy-20261009T044654Z/windows/install-receipt.json`,
  not the older `windows-typing-20261009T040502Z` receipt. This identifies the
  on-disk executable; it does not establish capture or keyboard behavior.
- The combined process/hash inspection then exited 1 because `quser` was
  unavailable. The preceding process and hash results were returned.

## Desktop-observation blocker

- The `computer_use` tool returned `Computer use is currently supported only
  on Windows`, reporting the current tool platform as Linux.
- CodeTether agent discovery returned no local or authenticated LAN peers.
- Windows has `CodeTether.Agent.Local_1.0.3246.50251_x64__enkasvdns50r0`, with
  an existing process in session 1. Both the CLI alias invocation and a direct
  `windows computer-use-worker` status invocation were denied with
  `Access is denied` over SSH (exit 1). No desktop-worker result was produced.
- No permissions, application installs, running processes, or pairing state
  were changed to work around these failures.

## Verification remaining — not-run

Current paired-session capture, fresh model completion, visible typing preview,
and actual insertion into the harmless focused Windows field were not run.