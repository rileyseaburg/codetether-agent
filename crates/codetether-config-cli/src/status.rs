//! Render effective execution policy alongside configuration.

use anyhow::Result;
use codetether_config::{ApprovalPolicy, Config, TrustPolicyStatus};

pub(super) fn print_effective(config: &Config) -> Result<()> {
    let status = TrustPolicyStatus::from_config(config);
    println!("\n# Effective execution policy");
    print!("{}", toml::to_string_pretty(&status)?);
    println!("prompt_behavior = \"{}\"", behavior(status.approval_policy));
    println!("fewer_prompts = \"codetether config --set access_mode=approve\"");
    println!("no_prompts = \"codetether config --set access_mode=full\"");
    Ok(())
}

fn behavior(policy: ApprovalPolicy) -> &'static str {
    match policy {
        ApprovalPolicy::Untrusted | ApprovalPolicy::OnRequest => "mutating tools require approval",
        ApprovalPolicy::OnFailure => "tools run unless escalation is needed",
        ApprovalPolicy::Never => "approval prompts are disabled",
    }
}
