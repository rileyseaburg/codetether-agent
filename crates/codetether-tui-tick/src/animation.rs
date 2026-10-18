use std::time::{Duration, SystemTime, UNIX_EPOCH};
/// Animation epoch that advances only while `processing`.
///
/// While idle this remains `0`. During processing it advances once per second,
/// avoiding expensive full-chat redraws solely for decorative animation.
pub fn animation_epoch(processing: bool, playing: bool) -> u64 {
    let elapsed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    animation_epoch_at(processing, playing, elapsed)
}

pub fn animation_epoch_at(processing: bool, playing: bool, elapsed: Duration) -> u64 {
    if playing {
        elapsed.as_millis() as u64 / 250
    } else if processing {
        elapsed.as_secs()
    } else {
        0
    }
}

#[cfg(test)]
#[path = "animation_tests.rs"]
mod tests;
