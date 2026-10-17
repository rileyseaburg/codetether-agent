//! System allocator configuration resolution.

const ENV_ARENA_MAX: &str = "CODETETHER_MALLOC_ARENA_MAX";
const ENV_TRIM_KIB: &str = "CODETETHER_MALLOC_TRIM_KIB";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Settings {
    pub arena_max: i32,
    pub trim_bytes: i32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            arena_max: 4,
            trim_bytes: 128 * 1024,
        }
    }
}

impl Settings {
    pub fn load() -> Self {
        let default = Self::default();
        Self {
            arena_max: value(ENV_ARENA_MAX, default.arena_max, 1, 32),
            trim_bytes: value(ENV_TRIM_KIB, 128, 16, 1_048_576) * 1024,
        }
    }
}

pub(super) fn value(name: &str, default: i32, min: i32, max: i32) -> i32 {
    parse(std::env::var(name).ok().as_deref(), default, min, max)
}

pub(super) fn parse(raw: Option<&str>, default: i32, min: i32, max: i32) -> i32 {
    raw.and_then(|value| value.trim().parse().ok())
        .unwrap_or(default)
        .clamp(min, max)
}
