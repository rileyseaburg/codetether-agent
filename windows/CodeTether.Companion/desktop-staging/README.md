# Native Windows desktop adapter

`codetether-companion-desktop` is an internal Rust library used by the single
`codetether-companion` application in `../rust-shell`. Cargo uses the repository
root `Cargo.toml` and `Cargo.lock`; this directory is not a nested workspace.
The C# implementation remains reference-only.

## Public boundary

- `check_available()` checks the active interactive desktop without granting
  permission to capture it.
- `monitors()` returns bounded physical-pixel monitor descriptions.
- `validate_selection()` rejects a stale or changed selected monitor.
- `capture_selected(&Monitor, &AtomicBool)` returns a memory-only
  `CapturedFrame`, or an error. Non-Windows calls return `Error::Unsupported`.
- `CapturedFrame::jpeg()`, `width()` and `height()` provide the encoded bytes
  and dimensions. Debug output never includes image bytes.

## Capture implementation (COMP-004)

Only the selected monitor's exact physical rectangle is copied with GDI
`BitBlt`/`SRCCOPY`; the virtual-desktop union is not captured. Source geometry
is bounded at 34 million pixels. A thread-local DPI guard preserves physical
coordinates, and selection/desktop eligibility are rechecked around native
work. A top-down BGRA DIB is flushed before pixel access.

Bilinear BGRA-to-RGB resizing preserves aspect ratio, never upscales, and caps
the longest edge at 1600 pixels. JPEG encoding tries qualities 75, 55, 35 and
20. Each attempt writes through a cancellable writer capped at 524288 bytes;
if no attempt fits, capture returns `Error::FrameTooLarge`. No oversized frame
is returned. The JPEG encoder uses a Rust-1.85-compatible dependency.

The DIB borrows its device context so it is released first. Native handles
are cleaned up on error and success; source resources are released before
JPEG encoding. Adapter-owned DIB, RGB and JPEG buffers are cleared on drop.
This does not guarantee erasure of copies owned by Windows, display drivers
or temporary storage internal to the encoder. No capture is written to disk
or uploaded by this library.

## Host consent and cancellation contract

The library does not own pairing, consent, scheduling or upload. A host must:

1. Use a monitor chosen locally. Pairing plus monitor selection and the visible
   sharing notice authorize background phone requests; no Windows Start is required.
2. Allocate a fresh, initially false cancellation flag per operation; never
   reset or reuse it. Set it on Pause, Stop, unpair, exit or session/display
   changes. Cancellation is cooperative, not an interrupt of a native call.
3. Keep capture off the UI thread and discard results from an old session or
   consent generation, including changes during encoding or upload handoff.
4. Recheck eligibility, selection and consent before authenticated upload.
   Supply timestamps, trigger and request identifiers in the transport layer.

Desktop checks are snapshots, not synchronization against Windows transitions.
The phone cannot choose or broaden the shared monitor. The shell services owner
requests in the background after setup; Resume requests is only needed following
an explicit local Pause. Lock/disconnect suspends work until eligibility returns.

## Verification status

**Not-run:** builds, tests, static checks and interactive Windows verification.