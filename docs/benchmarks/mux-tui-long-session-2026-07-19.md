# Long-running mux TUI memory audit — 2026-07-19

## Reproduced failure

The installed binary `86655e715b01e56954446fea6d5c0a7109269064fa0310fcd3c9dbb6df32be31`
had an exact-hash TUI running inside mux session `spotless6` for ten hours.
Although its live frame was idle and correct after reattach, PID 1613167 used
one complete CPU core and repeatedly reached about 5.6 GiB RSS. Its saved
session was only 0.76 MiB with 312 messages, and the same transcript rendered
cold in 21.6 ms, ruling out retained PTY output and chat formatting as causes.

## Root cause evidence

The idle PID read 5,124,650 bytes over a ten-second sample while recall
sidecars changed. The workspace had 5,427 recall sidecars. Code inspection
showed every workspace recall search unconditionally scheduled a process-local
legacy backfill that loaded full historical session JSON. Multiple TUI
processes therefore repeated the same migration independently.

## Remediation and focused proof

- Legacy workspace backfill is disabled unless
  `CODETETHER_RECALL_BACKFILL=1` is set.
- Opt-in migration handles at most eight sessions per run.
- Files larger than 8 MiB are skipped by the background worker.
- Accepted JSON is decoded on the blocking pool, away from async TUI workers.
- The retained real-session render benchmark remains available as
  `profiles_real_saved_session`.

Focused policy and cap tests passed. The existing real-workspace recall test
searched 447 cataloged sessions, returned ten hits in 1,261 ms, and retained
24,712 KiB RSS. The already-running PID retains its old in-flight worker and
must be restarted to receive the fix.

## Fresh mux TUI proof

Debug binary `8b9cfffb0b2d8c96ff103ee3707004205c4de47869d02191566e53b350f840fa`
started mux session `mux-recall-proof-20260719` in the real SpotlessBinCo
workspace. TUI PID 2714953 completed a 37-message turn with multiple read-only
tool calls while detached. The saved transcript was 102,349 bytes.

After completion, a twenty-second detached sample averaged 3.8% CPU, including
one periodic sync burst. RSS stayed at 149,640–149,644 KiB, PSS at about
110,458 KiB, and anonymous memory at about 44,820 KiB. Three detach/reattach
cycles preserved the process and the final reattach restored the latest final
response and empty input box. Attach-time PTY replay remained bounded to the
existing 64 KiB limit and was presented with synchronized terminal output.
