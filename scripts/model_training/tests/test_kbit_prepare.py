"""Verify k-bit preparation freezes weights without upcasting to float32."""

import unittest

import torch

from torch import nn

from model_training.kbit_prepare import prepare


class _Stub(nn.Module):
    """Minimal model exposing the interfaces preparation relies on."""

    def __init__(self) -> None:
        super().__init__()
        self.embed = nn.Embedding(4, 8, dtype=torch.bfloat16)
        self.config = type('Config', (), {'use_cache': True})()
        self.checkpointing = False

    def get_input_embeddings(self) -> nn.Module:
        return self.embed

    def gradient_checkpointing_enable(self, **_kwargs: object) -> None:
        self.checkpointing = True


class KbitPrepareTest(unittest.TestCase):
    """Preparation must not inflate parameter precision."""

    def test_parameters_keep_bfloat16(self) -> None:
        model = prepare(_Stub())
        for parameter in model.parameters():
            self.assertEqual(parameter.dtype, torch.bfloat16)

    def test_base_weights_are_frozen(self) -> None:
        model = prepare(_Stub())
        self.assertFalse(any(p.requires_grad for p in model.parameters()))

    def test_checkpointing_and_cache_configured(self) -> None:
        model = prepare(_Stub())
        self.assertTrue(model.checkpointing)
        self.assertFalse(model.config.use_cache)


if __name__ == '__main__':
    unittest.main()
