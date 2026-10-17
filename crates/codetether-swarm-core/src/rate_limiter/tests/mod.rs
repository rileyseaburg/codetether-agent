//! Existing rate-limit contract tests.
use super::*;
mod test_is_approaching_limit;
mod test_rate_limit_info_from_headers;
mod test_recommended_delay_when_approaching_limit;
mod test_recommended_delay_with_retry_after;
