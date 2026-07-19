"""Apply repository Ruff rules to external Forgejo protocol files."""

import subprocess
from pathlib import Path

ROOT = Path("/home/riley/A2A-Server-MCP")
PATTERNS = (
    "a2a_server/agent_task_record.py",
    "a2a_server/agent_task_persistence.py",
    "a2a_server/forgejo_*.py",
    "a2a_server/task_auth_scope.py",
    "a2a_server/worker_heartbeat.py",
    "a2a_server/worker_identity_*.py",
    "a2a_server/worker_registration_identity.py",
    "a2a_server/worker_target_failure.py",
    "a2a_server/worker_target_validation.py",
    "a2a_server/worker_task_mutation.py",
    "a2a_server/worker_extended_claim.py",
    "a2a_server/worker_request_resource.py",
    "a2a_server/worker_lifecycle_authorization.py",
    "tests/forgejo_metadata.py",
    "tests/forgejo_verification_transport.py",
    "tests/worker_signed_request.py",
    "tests/agent_bridge_fixtures.py",
    "tests/test_agent_bridge_required_persistence.py",
    "tests/test_agent_bridge_save_result.py",
    "tests/test_forgejo_*.py",
    "tests/test_legacy_task_endpoint.py",
    "tests/worker_identity_headers.py",
    "tests/test_worker_identity_*.py",
    "tests/test_worker_heartbeat*.py",
    "tests/test_worker_registration*.py",
    "tests/test_worker_mutation_proof.py",
    "tests/test_worker_release_proof.py",
    "tests/test_worker_monitor_mutation_proof.py",
    "tests/test_worker_extended_heartbeat_proof.py",
    "tests/test_worker_extended_claim_guard.py",
    "tests/test_worker_unregister_proof.py",
    "tests/test_worker_target_validation.py",
)


def files() -> list[str]:
    """Resolve the exact focused file set without shell expansion."""
    resolved: set[str] = set()
    for pattern in PATTERNS:
        for path in ROOT.glob(pattern):
            resolved.add(str(path.relative_to(ROOT)))
    return sorted(resolved)


def main() -> None:
    """Apply safe lint fixes followed by canonical Ruff formatting."""
    focused = files()
    subprocess.run(
        ["python3", "-m", "ruff", "format", *focused],
        cwd=ROOT,
        check=True,
    )
    subprocess.run(
        ["python3", "-m", "ruff", "check", "--fix", *focused],
        cwd=ROOT,
        check=True,
    )


if __name__ == "__main__":
    main()