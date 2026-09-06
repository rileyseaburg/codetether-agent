# Computer-use process and capture safety

Native Windows desktop actions run in a persistent subprocess rather than the
agent process. Requests are serialized, preserving shadow state between calls.
The internal `windows computer-use-worker` command is dispatched before agent
configuration, telemetry, or provider startup; stdout is protocol-only.

## Failure semantics

- Requests are limited to 256 KiB and responses to 12 MiB.
- Queue acquisition is bounded to 125 seconds; exchanges to 120 seconds.
- Crash, EOF, broken framing, timeout, or cancellation discards the worker.
- The failed action is never automatically retried: a click/key may already
  have affected the application. The next request creates a fresh worker.
- Errors report uncertain effects and lost shadow state. Logs record action,
  PID, byte counts and observed exit status, not text or image payloads.
- EOF alone is not proof of OOM or an access violation.

## Images and GDI

Capture dimensions are checked before allocating pixels or GDI objects. The hard
budget is 33,554,432 pixels (128 MiB of BGRA bytes). Allocation is fallible;
invalid, overflowing or excessive dimensions return an error.

DCs, bitmaps and selection lifetimes use RAII. Bitmaps are deselected before
`GetDIBits`, and partial scanline reads are rejected. GDI's reserved alpha channel
is converted to opaque pixels, not transparent screenshot content.

Original-resolution PNG captures are retained at unique artifact paths. Model
attachments have at most a 2048-pixel longest edge and 2 MiB encoded size. The
`preview` object reports actual dimensions and image-to-original scale factors.
Original geometry and OCR boxes are not silently rewritten: multiply preview
coordinates by the reported scales, then apply the capture's screen origin when
using physical screen coordinates. Large previews are JPEG; small PNGs may be
retained unchanged. OCR input files are bounded to 64 MiB.

## Keyboard targeting

Physical `type_text` and `press_key` require an explicit live HWND whose root
window is currently foreground. Missing targets or changed focus produce errors
instead of typing into an unrelated application such as the agent terminal.
This is a focus check, not a guarantee against subsequent OS/user focus races.
Shadow input remains opt-in and never falls back to physical input.

These controls bound native work and reduce parent image pressure. They do not
prove the cause of an earlier unlogged exit or bound all accumulated session data.