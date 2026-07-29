"""Checkpoint cadence for local runs.

Local steps are far cheaper than rented A100 steps, so evaluation can be
frequent. Behaviour is scored at every checkpoint because validation loss
proved unable to detect the tool-call suppression that made earlier adapters
worse than the untuned base model.
"""

LOCAL_CHECKPOINTING = {
    'eval_strategy': 'steps',
    'eval_steps': 50,
    'save_strategy': 'steps',
    'save_steps': 50,
    'save_total_limit': 4,
    'load_best_model_at_end': False,
}
"""`load_best_model_at_end` stays off.

Selecting by `eval_loss` chose checkpoints whose measured tool-call rate was
0.125 against 0.875 for the base model. Checkpoints are ranked afterwards by
observed behaviour instead.
"""
