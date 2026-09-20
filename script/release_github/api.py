"""GitHub-only CLI transport; Forgejo credentials are never used for publication."""

import json
import os
import subprocess

REPOSITORY = "rileyseaburg/codetether-agent"


class GitHub:
    def __init__(self) -> None:
        token = os.environ.get("GITHUB_RELEASE_TOKEN", "").strip()
        if not token:
            raise RuntimeError("GITHUB_RELEASE_TOKEN is required; GitHub is the canonical release destination")
        self.environment = os.environ.copy()
        for name in ["GITHUB_TOKEN", "FORGEJO_TOKEN", "GH_DEBUG", "GITHUB_API_URL"]:
            self.environment.pop(name, None)
        self.environment.update(GH_HOST="github.com", GH_TOKEN=token, GH_PROMPT_DISABLED="1")

    def run(self, arguments: list[str]) -> str:
        result = subprocess.run(["gh", *arguments], env=self.environment,
                                text=True, capture_output=True)
        if result.returncode:
            diagnostic = result.stderr.replace(self.environment["GH_TOKEN"], "[REDACTED]")
            raise RuntimeError(f"GitHub publication command failed: {diagnostic}")
        return result.stdout

    def request(self, path: str, method: str = "GET", fields: dict | None = None) -> dict | list:
        arguments = ["api", "--hostname", "github.com", f"repos/{REPOSITORY}/{path}", "--method", method]
        for name, value in (fields or {}).items():
            flag = "-F" if isinstance(value, bool) else "-f"
            encoded = str(value).lower() if isinstance(value, bool) else str(value)
            arguments.extend([flag, f"{name}={encoded}"])
        return json.loads(self.run(arguments))

    def releases(self) -> list[dict]:
        pages = self.run(["api", "--hostname", "github.com", "--paginate", "--slurp",
                          f"repos/{REPOSITORY}/releases?per_page=100"])
        return [release for page in json.loads(pages) for release in page]