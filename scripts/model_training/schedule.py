"""Optimizer schedule and evaluation cadence."""

ACCUMULATION_STEPS = 16

WARMUP_STEPS = 60
"""Explicit warmup steps.

`warmup_ratio` is deprecated in transformers and removed in v5.2. With
40,716 examples at an effective batch of 16, one epoch is roughly 2,500
optimizer steps, so 60 steps approximates the previous 3 percent ratio.
"""

EVAL_STEPS = 100
"""Evaluation and checkpoint interval in optimizer steps.

`load_best_model_at_end` requires a checkpoint at every evaluation point, so
this value bounds how much work a preemption can destroy. At a measured
17.26 s/it a 500 step interval exposed 2.4 hours of progress on preemptible
capacity; 100 steps bounds that to roughly 29 minutes.
"""

MAX_EVAL_SAMPLES = 400
"""Cap validation examples per evaluation pass.

A 400 example subset estimates loss closely enough to select checkpoints
while cutting each pass from about 19 minutes to about 3.
"""
