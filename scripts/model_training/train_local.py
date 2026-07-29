"""Fine-tune on a local GPU and score behaviour at every checkpoint.

Answers one question: does supervising emitted tool calls raise the tool-call
rate above the untuned base model's measured 0.875? Earlier adapters trained
on prose-only completions scored 0.125 and 0.250.
"""

import argparse

from pathlib import Path

from trl import SFTTrainer

from .adapter_setup import configure
from .behaviour_callback import BehaviourCallback
from .data_loader import splits
from .local_config import build_local
from .quantized_model import load


def main() -> None:
    """Train the adapter, recording behaviour beside each checkpoint."""
    parser = argparse.ArgumentParser()
    parser.add_argument('--train', type=Path, required=True)
    parser.add_argument('--validation', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--epochs', type=float, default=1.0)
    values = parser.parse_args()
    model, tokenizer = load()
    prepared, peft_config = configure(model)
    train, validation = splits(values.train, values.validation)
    trainer = SFTTrainer(
        model=prepared,
        args=build_local(values.output, values.epochs),
        train_dataset=train,
        eval_dataset=validation,
        processing_class=tokenizer,
        peft_config=peft_config,
    )
    trainer.add_callback(BehaviourCallback(values.output, tokenizer))
    trainer.train()
    trainer.save_model(str(values.output / 'final-adapter'))
    print('training complete')


if __name__ == '__main__':
    main()
