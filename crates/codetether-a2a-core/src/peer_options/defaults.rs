//! Automatic peer startup defaults.
use super::SpawnOptions;
impl SpawnOptions {
    /// Zero-config defaults: wildcard host, auto port/name, mDNS, intro on.
    pub fn auto() -> Self {
        Self {
            name: None,
            hostname: "0.0.0.0".to_string(),
            port: 0,
            public_url: None,
            description: None,
            peer: Vec::new(),
            discovery_interval_secs: 15,
            auto_introduce: true,
            mdns: true,
        }
    }
}
