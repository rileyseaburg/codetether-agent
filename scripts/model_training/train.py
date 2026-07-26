"""Run a real 4-bit CodeTether supervised fine-tune."""

import json
import os

import torch

from trl import SFTTrainer

from .adapter_setup import configure
from .data_loader import splits
from .quantized_model import load
from .run_manifest import write
from .train_args import parse
from .trainer_config import build as configuration


def main() -> None:
    """Evaluate the base, train the adapter, and evaluate the result."""
    settings = parse()
    settings.output.mkdir(parents=True, exist_ok=True)
    os.environ.setdefault('TOKENIZERS_PARALLELISM', 'false')
    torch.manual_seed(42)
    model, tokenizer = load()
    model, peft_config = configure(model, settings.adapter)
    train_data, validation_data = splits(settings.train, settings.validation)
    trainer = SFTTrainer(
        model=model,
        args=configuration(settings.output, settings.epochs, settings.masked),
        train_dataset=train_data,
        eval_dataset=validation_data,
        processing_class=tokenizer,
        peft_config=peft_config,
    )
    baseline = trainer.evaluate()
    checkpoint = str(settings.resume) if settings.resume else None
    result = trainer.train(resume_from_checkpoint=checkpoint)
    final = trainer.evaluate()
    adapter = settings.output / 'final-adapter'
    trainer.save_model(str(adapter))
    tokenizer.save_pretrained(adapter)
    write(
        settings.output / 'run-manifest.json', baseline, final, result.metrics
    )
    print(
        json.dumps(
            {'adapter': str(adapter), 'baseline': baseline, 'final': final}
        )
    )


if __name__ == '__main__':
    main()
