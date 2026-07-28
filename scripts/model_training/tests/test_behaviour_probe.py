"""Verify behaviour scoring detects the v2 failure modes."""

import unittest

from model_training.behaviour_probe import score
from model_training.behaviour_prompts import PROMPTS


class _Tokenizer:
    """Minimal tokenizer satisfying the probe's interface."""

    pad_token_id = 0

    def apply_chat_template(
        self, messages: list[dict[str, str]], **_: object
    ) -> str:
        return str(messages[0]['content'])

    def __call__(
        self, text: str, return_tensors: str = 'pt'
    ) -> dict[str, object]:
        return _Batch()

    def decode(self, _ids: object, skip_special_tokens: bool = False) -> str:
        return self.reply


class _Batch(dict):
    """Stand-in for a tokenizer batch with a device transfer."""

    def __init__(self) -> None:
        super().__init__(input_ids=_Ids())

    def to(self, _device: object) -> '_Batch':
        return self


class _Ids:
    shape = (1, 1)

    def __getitem__(self, _item: object) -> '_Ids':
        return self


class _Model:
    device = 'cpu'

    def generate(self, **_kwargs: object) -> list[list[int]]:
        return [[0]]


class BehaviourProbeTest(unittest.TestCase):
    """Empty output and silent tool use must both be measurable."""

    def test_empty_replies_are_counted(self) -> None:
        tokenizer = _Tokenizer()
        tokenizer.reply = '   '
        result = score(_Model(), tokenizer)
        self.assertEqual(result['empty_rate'], 1.0)
        self.assertEqual(result['probes'], len(PROMPTS))

    def test_tool_calls_are_detected(self) -> None:
        tokenizer = _Tokenizer()
        tokenizer.reply = '<tool_call>{"name": "read"}</tool_call>'
        result = score(_Model(), tokenizer)
        self.assertEqual(result['tool_call_rate'], 1.0)
        self.assertEqual(result['empty_rate'], 0.0)


if __name__ == '__main__':
    unittest.main()
