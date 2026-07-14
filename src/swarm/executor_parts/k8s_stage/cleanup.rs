//! Residual pod cleanup and deterministic result ordering.

use super::{pod_cleanup, state::State};

pub(super) async fn finish(state: &mut State<'_>) {
    if let Some(promoted) = &state.promoted {
        state
            .results
            .sort_by_key(|result| usize::from(&result.subtask_id != promoted));
    }
    let residual = state.active.keys().cloned().collect::<Vec<_>>();
    for id in residual {
        pod_cleanup::delete(state, &id, "stage cleanup").await;
    }
}
