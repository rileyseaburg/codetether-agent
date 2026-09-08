//! Automatic preflight regression suite and shared process fixtures.

use super::{blocked, budget, check, cooldown, scan};

#[path = "jsx_fixture.rs"]
mod jsx_fixture;
#[path = "jsx_rejection_tests.rs"]
mod jsx_rejection_tests;
#[path = "jsx_tests.rs"]
mod jsx_tests;
#[cfg(unix)]
#[path = "latency_fixture.rs"]
mod latency_fixture;
#[cfg(unix)]
#[path = "latency_tests.rs"]
mod latency_tests;
#[path = "real_project_tests.rs"]
mod real_project_tests;
#[cfg(unix)]
#[path = "startup_tests.rs"]
mod startup_tests;
#[path = "test_wait.rs"]
mod test_wait;
#[path = "tests.rs"]
mod tests;
#[cfg(unix)]
#[path = "warmup_tests.rs"]
mod warmup_tests;
