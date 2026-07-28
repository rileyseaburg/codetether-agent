"""Keep adapter dtype aligned with the frozen base throughout training.

Aligning once after constructing the trainer was not enough: TRL rebuilds
the adapter during its own setup, so a float32 adapter reappeared over
bfloat16 base weights and raised
`RuntimeError: expected scalar type BFloat16 but found Float` at the first
backward pass. Realigning on train begin and before each evaluation covers
every point where the adapter can be recreated.
"""

from transformers import TrainerCallback

from .adapter_dtype import align


class DtypeAlignCallback(TrainerCallback):
    """Realign trainable parameter dtypes at each lifecycle boundary."""

    def on_train_begin(
        self, args: object, state: object, control: object, **kwargs: object
    ) -> None:
        """Align before the first optimizer step."""
        model = kwargs.get('model')
        if model is not None:
            align(model)

    def on_evaluate(
        self, args: object, state: object, control: object, **kwargs: object
    ) -> None:
        """Align before generation-based evaluation."""
        model = kwargs.get('model')
        if model is not None:
            align(model)
