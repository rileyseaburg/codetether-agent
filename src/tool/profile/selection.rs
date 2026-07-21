//! Profile selection from provider identity and environment configuration.

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

pub(super) fn use_coding_profile(provider: &str) -> bool {
    resolve(requested(), provider)
}

fn resolve(requested: RequestedProfile, provider: &str) -> bool {
    match requested {
        RequestedProfile::Coding => true,
        RequestedProfile::MuxManager => false,
        RequestedProfile::Full | RequestedProfile::Unknown => false,
        RequestedProfile::Automatic => is_codex_provider(provider),
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

fn is_codex_provider(provider: &str) -> bool {
    matches!(provider, "openai-codex" | "codex" | "chatgpt")
}

#[cfg(test)]
#[path = "selection_tests.rs"]
mod tests;
