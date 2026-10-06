//! Environment parsing for the native CUDA provider.

pub(super) fn first_env(keys: &[&str]) -> Option<String> {
    keys.iter().find_map(|k| std::env::var(k).ok())
}

pub(super) fn parse_env_f32(keys: &[&str], default: f32) -> f32 {
    first_env(keys)
        .and_then(|v| v.parse::<f32>().ok())
        .unwrap_or(default)
}

pub(super) fn parse_env_usize(keys: &[&str], default: usize) -> usize {
    first_env(keys)
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(default)
}

pub(super) fn parse_env_u64(keys: &[&str], default: u64) -> u64 {
    first_env(keys)
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(default)
}
