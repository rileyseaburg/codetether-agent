//! Shared, copyable provider-login guidance for CLI help and recovery messages.

pub(crate) const WORKFLOW: &str = "Vault/provider setup (PowerShell, bash, or zsh):
  1. codetether vault url https://vault.spotlessbinco.com
     Replace the URL if you use a different Vault server.
  2. codetether vault login token
     Paste your Vault token at the prompt; or use: codetether vault login device
  3. codetether vault status
     Check that authenticated is true.
  4. codetether models
     Confirm that models are listed before starting work.

-t/--token is for the A2A/MCP control plane, NOT Vault.
Saved Vault login takes precedence over VAULT_TOKEN. CODETETHER_VAULT_SOURCE=env
or VAULT_ROLE selects environment/workload authentication instead.";
