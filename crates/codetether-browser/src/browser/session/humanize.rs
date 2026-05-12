//! Human-like timing and input helpers.
//!
//! Browsers (and bot-detection stacks in front of them) treat input as
//! suspicious when it arrives with machine-perfect timing, no mouse
//! movement, and click coordinates on the dead center of every element.
//! These helpers add small stochastic delays and coordinate jitter so the
//! actions look like a person at a keyboard.

use std::time::Duration;
use tokio::time::sleep;

use rand::RngExt;

/// Sleep a human-plausible inter-keystroke delay: mostly 50–140 ms with
/// an occasional ~400 ms pause simulating a word boundary or thought.
pub(super) async fn keystroke_delay() {
    let ms: u64 = {
        let mut rng = rand::rng();
        if rng.random_range(0..100) < 3 {
            rng.random_range(300..=650)
        } else {
            rng.random_range(50..=140)
        }
    };
    sleep(Duration::from_millis(ms)).await;
}

/// Small delay between mouse-down and mouse-up in a click, so the press is
/// detectable as a real gesture rather than an instantaneous toggle.
pub(super) async fn click_hold_delay() {
    let ms: u64 = {
        let mut rng = rand::rng();
        rng.random_range(40..=120)
    };
    sleep(Duration::from_millis(ms)).await;
}

/// Brief settle delay after a focus or scroll before the next action. Real
/// users don't start typing the instant a field gains focus.
pub(super) async fn settle_delay() {
    let ms: u64 = {
        let mut rng = rand::rng();
        rng.random_range(80..=220)
    };
    sleep(Duration::from_millis(ms)).await;
}

/// Jitter an (x, y) viewport coordinate by up to ±`radius` pixels on each
/// axis. Keeps the point inside the original element when callers pass the
/// element's half-extent as the radius.
pub(super) fn jitter_point(x: f64, y: f64, radius: f64) -> (f64, f64) {
    if radius <= 0.0 {
        return (x, y);
    }
    let mut rng = rand::rng();
    let dx: f64 = rng.random_range(-radius..=radius);
    let dy: f64 = rng.random_range(-radius..=radius);
    (x + dx, y + dy)
}
