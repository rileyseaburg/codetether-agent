//! Stable fallback assignment for unrecognized agent names.
use crate::AgentProfile;
use crate::fallback_profiles::*;

pub(crate) fn fallback_profile(name: &str) -> AgentProfile {
    let profiles = [
        profile_navigator(),
        profile_vector(),
        profile_signal(),
        profile_kernel(),
    ];
    let hash = fnv1a(name);
    profiles[hash as usize % profiles.len()].clone()
}

fn fnv1a(s: &str) -> u64 {
    let mut h: u64 = 2_166_136_261;
    for b in s.bytes() {
        h = (h ^ u64::from(b)).wrapping_mul(16_777_619);
    }
    h
}
