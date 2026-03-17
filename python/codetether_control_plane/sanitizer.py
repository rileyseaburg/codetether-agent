"""Secrets sanitizer for redacting sensitive values from agent output logs.

This module provides utilities to prevent accidental exposure of credentials
in monitor logs by redacting GitHub tokens, credential-bearing URLs, and
Authorization headers.
"""

import re
from typing import Optional


# GitHub token patterns
# - ghp_: GitHub personal access tokens (classic) - 40 chars total
# - gho_: GitHub OAuth access tokens - 40 chars total
# - ghu_: GitHub user-to-server tokens - 40 chars total
# - ghs_: GitHub server-to-server tokens - 40 chars total
# - github_pat_: GitHub fine-grained personal access tokens - 82+ chars
# Use flexible length matching for real-world token variations
GITHUB_TOKEN_PATTERN = re.compile(
    r'(ghp_[A-Za-z0-9]{35,}|gho_[A-Za-z0-9]{35,}|ghu_[A-Za-z0-9]{35,}|ghs_[A-Za-z0-9]{35,}|github_pat_[A-Za-z0-9_]{50,})'
)

# Authorization Bearer token pattern
BEARER_TOKEN_PATTERN = re.compile(
    r'\bAuthorization:\s*Bearer\s+[A-Za-z0-9._~+/=-]+',
    re.IGNORECASE
)

# x-access-token URL pattern (GitHub App installation tokens in URLs)
# Also handles URLs with port numbers (e.g., github.com:8443)
X_ACCESS_TOKEN_URL_PATTERN = re.compile(
    r'(https?://[^:@\s]*)(:x-access-token:)([^@\s]+)(@)',
    re.IGNORECASE
)

# Generic credential-bearing URL pattern (user:pass@host or token@host)
# Matches patterns like https://user:password@host or https://token@host
CREDENTIAL_URL_PATTERN = re.compile(
    r'(https?://)([^:\s]+)(:[^@\s]+)?(@)([^\s/]+)',
    re.IGNORECASE
)

# Replacement strings
REDACTED_TOKEN = "[REDACTED_TOKEN]"
REDACTED_BEARER = "Authorization: Bearer [REDACTED]"
REDACTED_CREDENTIAL = "[REDACTED]"


def _redact_github_tokens(text: str) -> str:
    """Redact GitHub token patterns."""
    return GITHUB_TOKEN_PATTERN.sub(REDACTED_TOKEN, text)


def _redact_bearer_tokens(text: str) -> str:
    """Redact Authorization Bearer headers."""
    return BEARER_TOKEN_PATTERN.sub(REDACTED_BEARER, text)


def _redact_access_token_urls(text: str) -> str:
    """Redact x-access-token credentials in URLs."""
    return X_ACCESS_TOKEN_URL_PATTERN.sub(
        lambda m: f"{m.group(1)}:x-access-token:{REDACTED_CREDENTIAL}{m.group(4)}",
        text
    )


def _redact_credential_urls(text: str) -> str:
    """Redact generic credential-bearing URLs.

    This catches patterns like:
    - https://user:password@host
    - https://token@host
    """
    def replace_credential(match: re.Match) -> str:
        scheme = match.group(1)
        # host = match.group(5)
        return f"{scheme}{REDACTED_CREDENTIAL}@{match.group(5)}"

    return CREDENTIAL_URL_PATTERN.sub(replace_credential, text)


def redact_secrets(text: str) -> str:
    """Redact sensitive secrets from text.

    This function identifies and redacts:
    - GitHub tokens (ghp_, gho_, ghu_, ghs_, github_pat_)
    - Authorization Bearer headers
    - Credential-bearing URLs (x-access-token, user:pass@host)

    Args:
        text: The text to sanitize.

    Returns:
        The text with secrets replaced by redaction placeholders.

    Example:
        >>> redact_secrets("Token: ghp_1234567890abcdefghijklmnopqrstuvwxYZ")
        'Token: [REDACTED_TOKEN]'
        >>> redact_secrets("curl -H 'Authorization: Bearer secret123' url")
        "curl -H 'Authorization: Bearer [REDACTED]' url"
    """
    if not text:
        return text

    # Apply redactions in order of specificity
    result = _redact_github_tokens(text)
    result = _redact_bearer_tokens(result)
    result = _redact_access_token_urls(result)
    # Skip generic credential URL pattern to avoid double-redacting
    # what x-access-token already handled

    return result


def sanitize_agent_output(output: str, max_length: Optional[int] = None) -> str:
    """Sanitize agent output for safe logging.

    This is the main entry point for sanitizing agent output before
    writing to monitor logs. It redacts secrets and optionally truncates.

    Args:
        output: The agent output text to sanitize.
        max_length: Optional maximum length for the output. If provided,
                   the output will be truncated to this length after
                   sanitization.

    Returns:
        Sanitized output safe for logging.

    Example:
        >>> sanitize_agent_output("Using token ghp_abc...")
        'Using token [REDACTED_TOKEN]...'
        >>> sanitize_agent_output("Cloning https://x-access-token:token123@github.com/repo")
        'Cloning https://x-access-token:[REDACTED]@github.com/repo'
    """
    if not output:
        return output

    result = redact_secrets(output)

    if max_length is not None and len(result) > max_length:
        result = result[:max_length] + "..."

    return result
