//! Profile selection from provider identity and environment configuration.

#[path = "selection_provider.rs"]
mod provider_rules;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum RequestedProfile {
    Automatic,
    Coding,
    Full,
    MuxManager,
    Unknown,
}

impl RequestedProfile {
    pub(super) fn is_coding(self) -> bool {
        self == Self::Coding
    }

    pub(super) fn is_mux_manager(self) -> bool {
        self == Self::MuxManager
    }
}

pub(super) fn requested() -> RequestedProfile {
    let value = std::env::var("CODETETHER_TOOL_PROFILE").ok();
    parse(value.as_deref())
}

pub(super) fn use_discovery_profile(provider: &str) -> bool {
    provider_rules::resolve_discovery(requested(), provider)
}

pub(super) fn use_coding_profile(provider: &str) -> bool {
    resolve(requested(), provider)
}

fn resolve(requested: RequestedProfile, provider: &str) -> bool {
    match requested {
        RequestedProfile::Coding => true,
        RequestedProfile::MuxManager => false,
        RequestedProfile::Full | RequestedProfile::Unknown => false,
        RequestedProfile::Automatic => provider_rules::prefers_coding(provider),
    }
}

fn parse(value: Option<&str>) -> RequestedProfile {
    match value.map(str::trim).map(str::to_ascii_lowercase).as_deref() {
        None | Some("") => RequestedProfile::Automatic,
        Some("lean" | "coding" | "codex") => RequestedProfile::Coding,
        Some("full" | "all") => RequestedProfile::Full,
        Some("mux-manager" | "mux_manager") => RequestedProfile::MuxManager,
        Some(_) => RequestedProfile::Unknown,
    }
}

#[cfg(test)]
#[path = "selection_tests.rs"]
mod tests;
