//! delegation::state contracts and policy operations.
//! `DelegationState` core data structure and key construction.

use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;

use super::beta::BetaPosterior;

use super::config::DelegationConfig;

use super::env::env_enabled_override;

use crate::relevance::Bucket;
mod type_delegation_state;
pub use type_delegation_state::DelegationState;
mod impl_delegation_state_enabled_1;
mod impl_delegation_state_key_2;
mod impl_delegation_state_with_config_0;
