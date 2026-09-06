# Native Windows OCR and shadow input

These features use the Rust `windows` bindings and native Windows APIs. Python,
`pytesseract`, `winrt` Python packages, Tesseract executables, and PowerShell are
not part of the OCR/input execution path. The Windows installer owns setup:
it supplies package identity, provisions OCR language support, and checks the
packaged app before declaring readiness. Use supported Windows 10/11.

Microsoft officially requires package identity for desktop `Windows.Media.Ocr`
use. Native OCR therefore reports unavailable for a bare unpackaged executable,
not a misleading success based solely on installed Python packages or DLLs.
The installer can use a signed release package or provision a locally signed
package with explicit Windows consent; it must not disable signature checks.
Language servicing and certificate trust can require the normal UAC prompt.

## OCR

Check actual recognizer availability, not Python package availability:

```json
{"action":"ocr_status"}
```

The result reports `runtime_available`, `available`, installed recognizer
languages, maximum image dimensions, and whether a user-profile recognizer can
be created (falling back to an installed recognizer). Readiness additionally
requires package identity and a successful in-memory native recognition probe.
Setup belongs to the installer, not an unexpected elevation during a tool call.

```json
{"action":"ocr","hwnd":123456,"language":"en-US"}
```

```json
{"action":"ocr","path":"C:\\screenshots\\inspector.png","language":"en-US"}
```

Omit both `path` and `hwnd` to capture the desktop. Supplying both is rejected.
Omit `language` to use Windows' user-profile recognizer. OCR runs on a blocking
thread with a balanced WinRT apartment, keeping the async executor responsive.

Results contain text, lines, words, bounding boxes, language, image dimensions,
and optional text angle. Boxes use **image-relative pixels**. Captures also
report physical-screen origins: add that origin to a box to get screen pixels.
Window captures include the non-client frame; these are not client coordinates.
File-based OCR makes no claim about a window or screen origin.

Images are not silently resized, cropped, or EXIF-rotated. Oversized inputs are
rejected against `OcrEngine.MaxImageDimension`; explicitly crop/resize or capture
a smaller window. Occluded/minimized window captures may be incomplete.

## Shadow mouse and keyboard

Physical mode remains the default. Shadow mode is an opt-in logical input path:

```json
{"action":"click","input_mode":"shadow","hwnd":123456,"client_area":true,"x":120,"y":80}
```

```json
{"action":"type_text","input_mode":"shadow","hwnd":123456,"text":"metadata"}
```

```json
{"action":"press_key","input_mode":"shadow","hwnd":123456,"key":"ENTER"}
```

Shadow input posts HWND-targeted Win32 messages. It does not call `SendInput`,
move the physical cursor, synthesize global keyboard state, or explicitly
activate a window. It is **not a second hardware cursor or desktop session**.
Target the actual child-control HWND when an application requires it.

Coordinates use physical pixels: `client_area:true` means client-relative;
otherwise they are relative to the outer window. Native message packing checks
signed 16-bit limits. Drags and text sizes are bounded. Simple navigation keys
and Unicode text are supported; modifier chords are rejected rather than
silently injected into the real keyboard.

`status` reports logical state for the HWND. `stop` releases only shadow-held
messages for that HWND, serialized after any already-running shadow action.
Results report `messages_queued` and `application_effect_unverified`: queue
acceptance is not proof that the application reacted. UIPI, elevated windows,
raw-input applications, and focus-dependent controls may reject or ignore the
messages. Recipients may themselves change focus; concurrent user input remains
possible. There is no automatic fallback to physical input.