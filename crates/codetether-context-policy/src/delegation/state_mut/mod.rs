//! delegation::state_mut contracts and policy operations.
//! `DelegationState` mutation and scoring methods.

use super::beta::BetaPosterior;

use super::state::DelegationState;

use crate::relevance::Bucket;
mod impl_delegation_state_delegate_to_3;
mod impl_delegation_state_ensure_0;
mod impl_delegation_state_score_1;
mod impl_delegation_state_update_2;
