"""Evaluation and checkpoint cadence for long training runs.

A full validation pass over 2,505 examples took 19 minutes on an A100
80 GB. Evaluating every 250 steps spent 3.2 hours of a 19 hour run on
evaluation alone, so the interval is widened and the subset bounded in
`data_loader`.
"""

from .constants import EVAL_STEPS


CHECKPOINTING = {
    'eval_strategy': 'steps',
    'eval_steps': EVAL_STEPS,
    'save_strategy': 'steps',
    'save_steps': EVAL_STEPS,
    'save_total_limit': 3,
    'load_best_model_at_end': True,
    'metric_for_best_model': 'eval_loss',
    'greater_is_better': False,
}
"""Cadence shared by the trainer configuration.

`save_steps` must equal `eval_steps` because `load_best_model_at_end`
requires a checkpoint for every evaluation point.
"""
