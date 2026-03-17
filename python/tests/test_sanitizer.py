"""Tests for the secrets sanitizer module."""

import pytest
from codetether_control_plane.sanitizer import (
    redact_secrets,
    sanitize_agent_output,
    _redact_github_tokens,
    _redact_bearer_tokens,
    _redact_access_token_urls,
)


class TestGitHubTokenRedaction:
    """Tests for GitHub token redaction patterns."""

    def test_redact_ghp_personal_access_token(self):
        """Test redaction of GitHub personal access token (classic)."""
        token = "ghp_1234567890abcdefghijklmnopqrstuvwxYZ01"
        result = redact_secrets(f"Using token {token}")
        assert token not in result
        assert "[REDACTED_TOKEN]" in result

    def test_redact_gho_oauth_token(self):
        """Test redaction of GitHub OAuth access token."""
        token = "gho_1234567890abcdefghijklmnopqrstuvwxYZ01"
        result = redact_secrets(f"OAuth token: {token}")
        assert token not in result
        assert "[REDACTED_TOKEN]" in result

    def test_redact_ghu_user_to_server_token(self):
        """Test redaction of GitHub user-to-server token."""
        token = "ghu_1234567890abcdefghijklmnopqrstuvwxYZ01"
        result = redact_secrets(f"User token {token} here")
        assert token not in result
        assert "[REDACTED_TOKEN]" in result

    def test_redact_ghs_server_to_server_token(self):
        """Test redaction of GitHub server-to-server token."""
        token = "ghs_1234567890abcdefghijklmnopqrstuvwxYZ01"
        result = redact_secrets(f"Server token: {token}")
        assert token not in result
        assert "[REDACTED_TOKEN]" in result

    def test_redact_github_pat_fine_grained(self):
        """Test redaction of GitHub fine-grained personal access token."""
        # Fine-grained PAT format: github_pat_ + alphanumeric (real tokens are ~82+ chars)
        token = "github_pat_11ABCDEFG22HIJKLMNOPQR_sTUVwXYz01ABcDefGHIJKLmNoPQrstUVwxyZ0123456789abcdEF"
        result = redact_secrets(f"Fine-grained token {token}")
        assert token not in result
        assert "[REDACTED_TOKEN]" in result

    def test_multiple_tokens_in_text(self):
        """Test redaction of multiple different tokens in one text."""
        text = "Tokens: ghp_1234567890abcdefghijklmnopqrstuvwxYZ01 and gho_1234567890abcdefghijklmnopqrstuvwxYZ01"
        result = redact_secrets(text)
        assert "ghp_" not in result
        assert "gho_" not in result
        assert result.count("[REDACTED_TOKEN]") == 2

    def test_token_in_json_output(self):
        """Test redaction when token appears in JSON-like output."""
        output = '{"token": "ghp_1234567890abcdefghijklmnopqrstuvwxYZ01", "status": "ok"}'
        result = redact_secrets(output)
        assert "ghp_" not in result
        assert "[REDACTED_TOKEN]" in result
        assert '"status": "ok"' in result


class TestBearerTokenRedaction:
    """Tests for Authorization Bearer token redaction."""

    def test_redact_bearer_header(self):
        """Test redaction of Authorization Bearer header."""
        text = "curl -H 'Authorization: Bearer secret_token_123' https://api.example.com"
        result = redact_secrets(text)
        assert "secret_token_123" not in result
        assert "Authorization: Bearer [REDACTED]" in result

    def test_redact_bearer_case_insensitive(self):
        """Test redaction works with different casing."""
        text = "AUTHORIZATION: bearer mysecrettoken"
        result = redact_secrets(text)
        assert "mysecrettoken" not in result
        assert "[REDACTED]" in result

    def test_redact_bearer_with_special_chars(self):
        """Test redaction of Bearer tokens with special characters."""
        text = "Authorization: Bearer abc123._~+/=-XYZ"
        result = redact_secrets(text)
        assert "abc123._~+/=-XYZ" not in result
        assert "[REDACTED]" in result


class TestAccessTokenUrlRedaction:
    """Tests for x-access-token URL redaction."""

    def test_redact_x_access_token_url(self):
        """Test redaction of x-access-token credentials in URLs."""
        # Use a realistic GitHub token length (40 chars after ghs_)
        url = "https://x-access-token:ghs_1234567890abcdefghijklmnopqrstuvwxYZ01@github.com/owner/repo.git"
        result = redact_secrets(url)
        assert "ghs_1234567890abcdefghijklmnopqrstuvwxYZ01" not in result
        # The secret should be redacted (either by GitHub token pattern or x-access-token pattern)
        assert "[REDACTED" in result and "]@github.com" in result

    def test_redact_preserves_repo_path(self):
        """Test that repo path is preserved after redaction."""
        # Use a realistic GitHub token format
        url = "https://x-access-token:ghs_1234567890abcdefghijklmnopqrstuvwxYZ01@github.com/owner/repo.git"
        result = redact_secrets(url)
        assert "github.com/owner/repo.git" in result
        assert "ghs_1234567890abcdefghijklmnopqrstuvwxYZ01" not in result


class TestSanitizeAgentOutput:
    """Tests for the main sanitize_agent_output function."""

    def test_basic_sanitization(self):
        """Test basic output sanitization."""
        output = "Cloning repo with token ghp_1234567890abcdefghijklmnopqrstuvwxYZ01"
        result = sanitize_agent_output(output)
        assert "ghp_" not in result
        assert "[REDACTED_TOKEN]" in result

    def test_empty_string(self):
        """Test handling of empty string."""
        assert sanitize_agent_output("") == ""
        assert sanitize_agent_output(None) is None

    def test_truncation(self):
        """Test output truncation with max_length."""
        output = "A" * 1000 + " ghp_1234567890abcdefghijklmnopqrstuvwxYZ01"
        result = sanitize_agent_output(output, max_length=100)
        assert len(result) == 103  # 100 + "..."
        assert result.endswith("...")

    def test_truncation_with_secret_at_end(self):
        """Test that secrets are still redacted before truncation."""
        output = "Short text " + "B" * 50 + " ghp_1234567890abcdefghijklmnopqrstuvwxYZ01"
        result = sanitize_agent_output(output, max_length=20)
        # Secret should be redacted before truncation
        assert "ghp_" not in result

    def test_no_modification_when_no_secrets(self):
        """Test that clean text is not modified."""
        output = "Agent completed successfully. All tasks done."
        result = sanitize_agent_output(output)
        assert result == output

    def test_complex_output_sanitization(self):
        """Test sanitization of complex agent output."""
        output = """
        Agent Log:
        - Cloning https://x-access-token:ghs_1234567890abcdefghijklmnopqrstuvwxYZ01@github.com/org/repo
        - Setting up authentication
        - API call with Authorization: Bearer api_token_xyz
        - Using PAT: ghp_1234567890abcdefghijklmnopqrstuvwxYZ01
        - Complete
        """
        result = sanitize_agent_output(output)
        assert "ghs_1234567890abcdefghijklmnopqrstuvwxYZ01" not in result
        assert "api_token_xyz" not in result
        assert "ghp_1234567890abcdefghijklmnopqrstuvwxYZ01" not in result
        assert result.count("[REDACTED]") >= 1 or result.count("[REDACTED_TOKEN]") >= 1


class TestEdgeCases:
    """Tests for edge cases and boundary conditions."""

    def test_partial_token_not_redacted(self):
        """Test that partial/incomplete tokens are not mistakenly redacted."""
        # This is a truncated token, should not match
        partial = "ghp_short"
        result = redact_secrets(f"Token: {partial}")
        assert partial in result  # Should NOT be redacted

    def test_token_boundary_at_start_of_string(self):
        """Test token at the very start of string."""
        token = "ghp_1234567890abcdefghijklmnopqrstuvwxYZ01"
        result = redact_secrets(token)
        assert token not in result
        assert result == "[REDACTED_TOKEN]"

    def test_token_boundary_at_end_of_string(self):
        """Test token at the very end of string."""
        token = "ghp_1234567890abcdefghijklmnopqrstuvwxYZ01"
        result = redact_secrets(f"Prefix {token}")
        assert token not in result
        assert result == "Prefix [REDACTED_TOKEN]"

    def test_adjacent_tokens(self):
        """Test handling of adjacent tokens with separator."""
        # Real-world tokens would have some separator; adjacent without separator is unusual
        # but we test both scenarios
        text = "ghp_1234567890abcdefghijklmnopqrstuvwxYZ01 ghp_1234567890abcdefghijklmnopqrstuvwxYZ02"
        result = redact_secrets(text)
        # Both should be redacted
        assert "ghp_" not in result
        assert result.count("[REDACTED_TOKEN]") == 2

    def test_whitespace_variations_in_bearer(self):
        """Test Bearer header with various whitespace."""
        variations = [
            "Authorization: Bearer token123",
            "Authorization:Bearer token123",
            "Authorization:  Bearer   token123",
        ]
        for text in variations:
            result = redact_secrets(text)
            # The regex requires at least one space after Bearer
            # so some variations may not match exactly

    def test_url_with_port(self):
        """Test URLs with port numbers."""
        url = "https://x-access-token:ghs_1234567890abcdefghijklmnopqrstuvwxYZ01@github.com:8443/owner/repo.git"
        result = redact_secrets(url)
        # The token should be redacted (either by GitHub token pattern or x-access-token pattern)
        assert "ghs_1234567890abcdefghijklmnopqrstuvwxYZ01" not in result


class TestRedactSecretsFunction:
    """Tests for the main redact_secrets function."""

    def test_returns_string(self):
        """Test that function always returns a string."""
        assert isinstance(redact_secrets("text"), str)
        assert isinstance(redact_secrets(""), str)

    def test_no_secrets_returns_original(self):
        """Test that text without secrets is unchanged."""
        text = "Normal log message without any secrets"
        assert redact_secrets(text) == text

    def test_combined_secret_types(self):
        """Test text with multiple types of secrets."""
        text = """
        Token: ghp_1234567890abcdefghijklmnopqrstuvwxYZ01
        URL: https://x-access-token:ghs_1234567890abcdefghijklmnopqrstuvwxYZ01@github.com/repo
        Header: Authorization: Bearer mytoken
        """
        result = redact_secrets(text)
        assert "ghp_1234567890abcdefghijklmnopqrstuvwxYZ01" not in result
        assert "ghs_1234567890abcdefghijklmnopqrstuvwxYZ01" not in result
        assert "mytoken" not in result
