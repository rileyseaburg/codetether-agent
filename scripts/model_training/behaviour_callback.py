"""Score behaviour at each checkpoint instead of trusting loss alone.

The v2 model improved validation loss from 3.4744 to 1.1585 while its
code-fix pass rate fell from 0.75 to 0.00 and half of all short prompts
returned nothing. Selecting checkpoints by `eval_loss` would repeat that
mistake, so generation behaviour is measured during training.
"""

import json

from pathlib import Path

from transformers import TrainerCallback

from .behaviour_probe import score
from .distributed import is_primary


class BehaviourCallback(TrainerCallback):
    """Record generation behaviour whenever the trainer evaluates."""

    def __init__(self, output: Path, tokenizer: object) -> None:
        self.path = output / 'behaviour.jsonl'
        self.tokenizer = tokenizer

    def on_evaluate(
        self,
        args: object,
        state: object,
        control: object,
        model: object = None,
        **kwargs: object,
    ) -> None:
        """Append behaviour metrics for the current step."""
        if model is None or not is_primary():
            return
        record = score(model, self.tokenizer)
        record['step'] = int(getattr(state, 'global_step', 0))
        self.path.parent.mkdir(parents=True, exist_ok=True)
        with self.path.open('a') as handle:
            handle.write(json.dumps(record, sort_keys=True) + '\n')
