//! CLI argument adapter for shared peer startup options.
use super::*;
impl From<&SpawnArgs> for SpawnOptions {
    fn from(args: &SpawnArgs) -> Self {
        Self {
            name: args.name.clone(),
            hostname: args.hostname.clone(),
            port: args.port,
            public_url: args.public_url.clone(),
            description: args.description.clone(),
            peer: args.peer.clone(),
            discovery_interval_secs: args.discovery_interval_secs,
            auto_introduce: args.auto_introduce,
            mdns: args.mdns,
        }
    }
}
