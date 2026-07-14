use std::time::{Duration, Instant};

use super::config::Config;
use super::state::{Actions, State};

fn config() -> Config {
    Config {
        warn_mib: 100,
        critical_mib: 300,
        sample: Duration::from_secs(1),
        trim: Duration::from_secs(30),
    }
}

#[test]
fn thresholds_fire_once_and_trim_is_rate_limited() {
    let mut state = State::default();
    let start = Instant::now();
    assert_eq!(state.observe(99, start, config()), Actions::default());
    assert_eq!(
        state.observe(300, start, config()),
        Actions {
            warn: true,
            critical: true,
            trim: true,
        }
    );
    assert_eq!(
        state.observe(400, start + Duration::from_secs(29), config()),
        Actions::default()
    );
    assert!(
        state
            .observe(400, start + Duration::from_secs(30), config())
            .trim
    );
}
