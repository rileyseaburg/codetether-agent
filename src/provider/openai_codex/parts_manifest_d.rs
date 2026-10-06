// Include manifest for live model discovery on the Codex provider.

include!("parts/discover_chatgpt_models.rs");
include!("parts/discovered_model_info.rs");

// Cross-process OAuth refresh coordination.
include!("parts/refresh_lock.rs");
include!("parts/rotate_credentials.rs");
include!("parts/sync_shared_credentials.rs");
include!("parts/vault_credential_load.rs");
