"""Verify trainable weights match the frozen base dtype."""

import unittest

import torch

from torch import nn

from model_training.adapter_dtype import align, base_dtype


class _Model(nn.Module):
    """Frozen bfloat16 base with a float32 trainable adapter."""

    def __init__(self) -> None:
        super().__init__()
        self.base = nn.Linear(4, 4, dtype=torch.bfloat16)
        self.adapter = nn.Linear(4, 4, dtype=torch.float32)
        for parameter in self.base.parameters():
            parameter.requires_grad = False


class AdapterDtypeTest(unittest.TestCase):
    """A float32 adapter on a bfloat16 base raised RuntimeError in backward."""

    def test_base_dtype_ignores_trainable_weights(self) -> None:
        self.assertEqual(base_dtype(_Model()), torch.bfloat16)

    def test_trainable_weights_are_cast_to_base(self) -> None:
        model = align(_Model())
        for parameter in model.adapter.parameters():
            self.assertEqual(parameter.dtype, torch.bfloat16)

    def test_frozen_weights_are_untouched(self) -> None:
        model = align(_Model())
        for parameter in model.base.parameters():
            self.assertEqual(parameter.dtype, torch.bfloat16)
            self.assertFalse(parameter.requires_grad)


if __name__ == '__main__':
    unittest.main()
