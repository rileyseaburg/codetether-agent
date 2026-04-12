from __future__ import annotations

import os
from pathlib import Path


def project_root() -> Path:
    return Path(__file__).resolve().parents[2]


def data_dir() -> Path:
    env_path = os.environ.get("BROWSERCTL_DATA_DIR")
    path = Path(env_path).expanduser() if env_path else project_root() / ".codetether-agent" / "browserctl"
    path.mkdir(parents=True, exist_ok=True)
    return path


def profile_dir() -> Path:
    env_path = os.environ.get("BROWSERCTL_PROFILE_DIR")
    path = Path(env_path).expanduser() if env_path else data_dir() / "profile"
    path.mkdir(parents=True, exist_ok=True)
    return path


def default_browser_path() -> str | None:
    env_path = os.environ.get("BROWSERCTL_EXECUTABLE")
    if env_path:
        return env_path
    candidates = [
        Path.home() / ".cache/ms-playwright/chromium-1208/chrome-linux64/chrome",
        Path.home() / ".cache/ms-playwright/chromium-1200/chrome-linux64/chrome",
        Path("/snap/bin/chromium"),
        Path("/usr/bin/chromium-browser"),
    ]
    for candidate in candidates:
        if candidate.exists():
            return str(candidate)
    return None
