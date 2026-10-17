# Linux desktop screenshots

`computer_use` supports `status` and whole-desktop `snapshot` on Linux, including
Ubuntu. Other desktop actions, window targeting, input automation, and OCR remain
Windows-only. Browser-page screenshots are a separate `browserctl` capability.

## Setup

Run CodeTether from a terminal in the logged-in graphical session. It inherits
that session's `DISPLAY`, `WAYLAND_DISPLAY`, and desktop authorization. An SSH,
container, or system-service process without access to the desktop cannot
capture it merely because a screenshot utility is installed.

Install a utility appropriate for your desktop; only one working backend is
needed:

| Session | Capture utilities |
| --- | --- |
| Ubuntu/GNOME | `gnome-screenshot` |
| KDE Plasma | `spectacle` |
| wlroots Wayland (for example, Sway) | `grim` |
| X11 | `gnome-screenshot`, `spectacle`, `scrot`, `maim`, or ImageMagick `import` |

For example, on Ubuntu with GNOME:

```sh
sudo apt install gnome-screenshot
```

Wayland compositor permissions still apply. `grim` is not a generic GNOME
Wayland fallback. Some desktop versions prohibit noninteractive screenshots or
require a permission prompt; this tool does not bypass those restrictions. X11
utilities are never used for a Wayland session, even when XWayland supplies a
`DISPLAY`, because its root window is not the whole Wayland desktop.
Capture subprocesses also have `DISPLAY` removed and GTK/Qt forced to Wayland
in Wayland sessions, so a utility cannot silently fall back to XWayland.

## Tool requests

Inspect capabilities and installed, session-compatible backends first:

```json
{"action":"status"}
```

`supported` describes implemented Linux support. `screen_capture_ready` means a
graphical session and a compatible utility were detected, not that permissions
have been verified. A snapshot is the actual capture check:

```json
{"action":"snapshot"}
```

A successful result includes the retained original PNG's `path`, dimensions,
backend and session type, plus an attached bounded preview. Originals live in a
private `codetether-snapshot-*` directory under the operating system's temporary
directory. Treat these files as sensitive; remove them when no longer needed.
Previews have a longest edge of at most 2048 pixels and an encoded-size limit of
2 MiB. Use the returned preview scale factors to map back to original pixels.

Linux coordinates are `captured_image_pixels`. Global desktop origin and cursor
position are unavailable; these coordinates are not an input-injection API.
`hwnd`, `app`, `window_title_contains`, `client_area`, and `viewport_child_hwnd`
selectors are rejected rather than silently capturing a different target.

Capture attempts use shell-free subprocesses with a 10-second timeout for each
backend. Failed attempts may fall back to another compatible utility. Errors
identify the attempted backend and give session/permission guidance.

## Focused verification

```sh
cargo test --lib tool::computer_use::
env -u WAYLAND_DISPLAY XDG_SESSION_TYPE=x11 xvfb-run -a -s '-screen 0 1280x720x24' \
  cargo test --lib live_x11_snapshot_returns_a_real_png_attachment -- --ignored --nocapture
```