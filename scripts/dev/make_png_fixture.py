#!/usr/bin/env python3
"""Build PNG fixtures and data URLs for TUI image-paste testing.

Writes a tiny 8x8 red PNG and a screenshot-sized incompressible PNG, each
with a matching ``data:image/png;base64,...`` file, so the clipboard paste
path can be exercised on a headless host with no display server. Also emits
an over-cap fixture to verify the size-limit error surfaces.
"""

import base64
import pathlib
import random
import struct
import sys
import zlib


DEFAULT_OUT = "/tmp/ct-image-test"


def chunk(tag: bytes, data: bytes) -> bytes:
    """Return one length-prefixed, CRC-suffixed PNG chunk."""
    body = tag + data
    crc = struct.pack(">I", zlib.crc32(body))
    return struct.pack(">I", len(data)) + body + crc


def rows_for(width: int, height: int, noise: bool) -> bytes:
    """Build raw RGB scanlines, optionally incompressible noise."""
    if not noise:
        red = b"\x00" + b"\xff\x00\x00" * width
        return red * height
    rng = random.Random(7)
    return b"".join(
        b"\x00" + bytes(rng.randrange(256) for _ in range(width * 3))
        for _ in range(height)
    )


def make_png(width: int = 8, height: int = 8, noise: bool = False) -> bytes:
    """Encode a minimal truecolor PNG."""
    ihdr = struct.pack(">IIBBBBB", width, height, 8, 2, 0, 0, 0)
    return (
        b"\x89PNG\r\n\x1a\n"
        + chunk(b"IHDR", ihdr)
        + chunk(b"IDAT", zlib.compress(rows_for(width, height, noise)))
        + chunk(b"IEND", b"")
    )


def emit(out: pathlib.Path, name: str, png: bytes) -> None:
    """Write ``name.png`` plus ``name.dataurl`` and report their sizes."""
    (out / f"{name}.png").write_bytes(png)
    url = "data:image/png;base64," + base64.b64encode(png).decode()
    (out / f"{name}.dataurl").write_text(url, encoding="utf-8")
    print(f"{name}: png_bytes={len(png)} dataurl_len={len(url)}")


def main() -> int:
    """Generate both fixtures under the requested output directory."""
    out = pathlib.Path(sys.argv[1] if len(sys.argv) > 1 else DEFAULT_OUT)
    out.mkdir(parents=True, exist_ok=True)
    emit(out, "red", make_png())
    emit(out, "big", make_png(640, 400, noise=True))
    # Exceeds the 10 MB decoded cap in image_clipboard::data_url.
    emit(out, "huge", make_png(2400, 1600, noise=True))
    return 0


if __name__ == "__main__":
    sys.exit(main())
