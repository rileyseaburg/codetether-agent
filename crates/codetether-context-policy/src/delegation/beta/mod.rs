//! delegation::beta contracts and policy operations.
//! Per-(agent, skill, bucket) Beta-Bernoulli posterior.

use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};
mod type_beta_posterior;
pub use type_beta_posterior::BetaPosterior;
mod impl_beta_posterior_from_self_confidence_0;
mod impl_beta_posterior_mean_1;
mod impl_beta_posterior_score_3;
mod impl_beta_posterior_update_4;
mod impl_beta_posterior_variance_2;
