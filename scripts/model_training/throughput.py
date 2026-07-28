"""Throughput settings that remove padding waste.

A measured run showed a mean of 3,333 real tokens padded to an 8,192 window,
so roughly 59 percent of every batch was padding and one epoch took 12.2
hours at 17.26 s/it.

`padding_free` flattens a batch into one continuous sequence instead of
padding, and unlike `packing` it does not merge separate samples, so
completion-only loss boundaries stay intact. It requires FlashAttention-2.
"""

MEAN_REAL_TOKENS = 3333
PADDED_WINDOW = 8192


def utilization() -> float:
    """Return the measured fraction of each padded window that carries data."""
    return MEAN_REAL_TOKENS / PADDED_WINDOW


def settings(flash_attention: bool) -> dict[str, object]:
    """Return trainer settings for the available attention kernel.

    Padding-free batching is only correct with FlashAttention-2; requesting
    it without that kernel silently degrades or errors, so it is gated.

    `group_by_length` is deliberately absent. Enabling it required a length
    column on the dataset, and that column broke training before the first
    optimizer step while the same configuration without it trained cleanly.
    Padding waste is a throughput cost; a run that cannot start is a total
    loss, so the working configuration is preferred until grouping is proven
    on a GPU.
    """
    if not flash_attention:
        return {'padding_free': False, 'packing': False}
    return {'padding_free': True, 'packing': False}
