"""Verify HuggingFace token resolution order and failure guidance."""

import os
import unittest

from unittest import mock

from model_training.hf_token import resolve_token


class HfTokenTest(unittest.TestCase):
    """Vault is primary; the environment is a local fallback."""

    def test_environment_fallback_when_vault_absent(self) -> None:
        env = {'HF_TOKEN': 'env-token'}
        with mock.patch.dict(os.environ, env, clear=True):
            self.assertEqual(resolve_token(), 'env-token')

    def test_vault_takes_priority(self) -> None:
        env = {
            'VAULT_ADDR': 'https://vault.example',
            'VAULT_TOKEN': 'vault-token',
            'HF_TOKEN': 'env-token',
        }
        with (
            mock.patch.dict(os.environ, env, clear=True),
            mock.patch(
                'model_training.hf_token.token_for', return_value='vault-value'
            ),
        ):
            self.assertEqual(resolve_token(), 'vault-value')

    def test_missing_everywhere_raises_actionable_error(self) -> None:
        with mock.patch.dict(os.environ, {}, clear=True):
            with self.assertRaises(SystemExit) as caught:
                resolve_token()
            self.assertIn('huggingface', str(caught.exception))


if __name__ == '__main__':
    unittest.main()
