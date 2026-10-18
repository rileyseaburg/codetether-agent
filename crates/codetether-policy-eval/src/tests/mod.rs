//! Regression tests for policy comparison semantics.
mod fixtures;
mod pareto_frontier_keeps_nondominated_points;
mod reuse_rate_is_zero_when_total_is_zero;
mod reuse_rate_round_trips_typical_values;
mod strictly_better_on_all_axes_dominates;
mod tied_on_all_does_not_dominate;
