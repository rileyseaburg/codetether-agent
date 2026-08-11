//! Construction of the local DNS-SD service description.

use super::{SERVICE_TYPE, sanitize_hostname};
use anyhow::{Context, Result};
use mdns_sd::ServiceInfo;
use std::{collections::HashMap, net::IpAddr};

pub(super) fn description(
    instance_name: &str,
    bind_port: u16,
    bound_addrs: &[IpAddr],
    collaboration_token: &str,
) -> Result<ServiceInfo> {
    let hostname = format!("{}.local.", sanitize_hostname(instance_name));
    ServiceInfo::new(
        SERVICE_TYPE,
        instance_name,
        &hostname,
        bound_addrs,
        bind_port,
        Some(properties(instance_name, collaboration_token)),
    )
    .context("Failed to construct mDNS ServiceInfo")
}

fn properties(instance_name: &str, collaboration_token: &str) -> HashMap<String, String> {
    HashMap::from([
        ("name".to_string(), instance_name.to_string()),
        ("path".to_string(), "/".to_string()),
        ("protocol".to_string(), "a2a-jsonrpc".to_string()),
        ("version".to_string(), env!("CARGO_PKG_VERSION").to_string()),
        ("auth".to_string(), collaboration_token.to_string()),
    ])
}
