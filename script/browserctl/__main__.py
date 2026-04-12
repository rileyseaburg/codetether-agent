from __future__ import annotations

import os

import uvicorn


def main() -> None:
    uvicorn.run(
        "script.browserctl.server:app",
        host=os.environ.get("BROWSERCTL_HOST", "127.0.0.1"),
        port=int(os.environ.get("BROWSERCTL_PORT", "4477")),
    )


if __name__ == "__main__":
    main()
