//! Plugin signing, content verification, and subprocess policy checks.
//!
//! Plugin registration fails closed unless the manifest signature is valid and
//! the declared content hash matches the bytes or file supplied by the caller.
//! Subprocess execution enforces pre-spawn policy checks and reports any
//! platform fallback that is only advisory.
//!
//! Built-in tools are trusted. Third-party plugins must be registered with
//! verified content before callers should execute them.

use super::{sandbox_limits, sandbox_network, sandbox_paths};
use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;

/// Manifest describing a plugin tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    /// Unique plugin identifier.
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Semantic version.
    pub version: String,
    /// SHA-256 hash of the plugin content (source or binary).
    pub content_hash: String,
    /// Who signed this manifest.
    pub signed_by: String,
    /// Hex-encoded HMAC-SHA256 signature of `id|version|content_hash`.
    pub signature: String,
    /// Allowed capabilities (e.g., "fs:read", "net:connect", "exec:shell").
    #[serde(default)]
    pub capabilities: Vec<String>,
    /// Maximum execution time in seconds.
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
}

fn default_timeout() -> u64 {
    30
}

/// Sandbox execution policy for a tool invocation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxPolicy {
    /// Whether filesystem access is allowed (and to which paths).
    pub allowed_paths: Vec<PathBuf>,
    /// Whether network access is allowed.
    pub allow_network: bool,
    /// Whether shell execution is allowed.
    pub allow_exec: bool,
    /// Maximum execution time in seconds.
    pub timeout_secs: u64,
    /// Maximum memory in bytes (0 = no limit).
    pub max_memory_bytes: u64,
}

impl Default for SandboxPolicy {
    fn default() -> Self {
        Self {
            allowed_paths: Vec::new(),
            allow_network: false,
            allow_exec: false,
            timeout_secs: 30,
            max_memory_bytes: 256 * 1024 * 1024, // 256 MB
        }
    }
}

/// Result of a sandboxed tool execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxResult {
    pub tool_id: String,
    pub success: bool,
    pub output: String,
    /// SHA-256 hash of the combined output for integrity verification.
    pub output_hash: String,
    pub exit_code: Option<i32>,
    pub duration_ms: u64,
    pub sandbox_violations: Vec<String>,
    /// Isolation gaps that are visible to callers instead of hidden.
    #[serde(default)]
    pub unsafe_fallbacks: Vec<String>,
}

/// The signing key used to verify plugin manifests.
#[derive(Clone)]
pub struct SigningKey {
    key: Arc<Vec<u8>>,
}

static GLOBAL_SIGNING_KEY: OnceLock<SigningKey> = OnceLock::new();

impl std::fmt::Debug for SigningKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SigningKey")
            .field("key_len", &self.key.len())
            .finish()
    }
}

impl SigningKey {
    /// Load the HMAC key from `CODETETHER_PLUGIN_SIGNING_KEY` or generate one.
    pub fn from_env() -> Self {
        let key = match std::env::var("CODETETHER_PLUGIN_SIGNING_KEY") {
            Ok(hex) if hex.len() >= 32 => {
                tracing::info!("Plugin signing key loaded from environment");
                hex.into_bytes()
            }
            _ => {
                let mut rng = rand::rng();
                let key: Vec<u8> = (0..32)
                    .map(|_| rand::RngExt::random::<u8>(&mut rng))
                    .collect();
                tracing::warn!(
                    "No CODETETHER_PLUGIN_SIGNING_KEY set — generated ephemeral key. \
                     Plugin signatures will not persist across restarts."
                );
                key
            }
        };
        Self { key: Arc::new(key) }
    }

    /// Load the process-wide signing key exactly once.
    pub fn shared() -> Self {
        GLOBAL_SIGNING_KEY.get_or_init(Self::from_env).clone()
    }

    /// Create with an explicit key (for tests).
    #[cfg(test)]
    pub fn with_key(key: Vec<u8>) -> Self {
        Self { key: Arc::new(key) }
    }

    /// Sign a manifest payload: `id|version|content_hash`.
    pub fn sign(&self, id: &str, version: &str, content_hash: &str) -> String {
        use hmac::{Hmac, Mac};
        type HmacSha256 = Hmac<Sha256>;

        let payload = format!("{}|{}|{}", id, version, content_hash);
        let mut mac = HmacSha256::new_from_slice(&self.key).expect("HMAC can take key of any size");
        mac.update(payload.as_bytes());
        let result = mac.finalize();
        hex::encode(result.into_bytes())
    }

    /// Verify a manifest signature.
    pub fn verify(&self, manifest: &PluginManifest) -> bool {
        let expected = self.sign(&manifest.id, &manifest.version, &manifest.content_hash);
        constant_time_eq(expected.as_bytes(), manifest.signature.as_bytes())
    }
}

/// Compute SHA-256 hash of file contents.
pub fn hash_file(path: &Path) -> Result<String> {
    let contents = std::fs::read(path)
        .with_context(|| format!("Failed to read file for hashing: {}", path.display()))?;
    let mut hasher = Sha256::new();
    hasher.update(&contents);
    Ok(hex::encode(hasher.finalize()))
}

/// Compute SHA-256 hash of byte content.
pub fn hash_bytes(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

/// Plugin registry — tracks registered and verified plugins.
#[derive(Debug)]
pub struct PluginRegistry {
    signing_key: SigningKey,
    /// Verified plugins: id -> manifest.
    plugins: Arc<RwLock<HashMap<String, PluginManifest>>>,
}

impl PluginRegistry {
    pub fn new(signing_key: SigningKey) -> Self {
        Self {
            signing_key,
            plugins: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn from_env() -> Self {
        Self::new(SigningKey::shared())
    }

    /// Register a plugin manifest only when unsafe content bypass is enabled.
    pub async fn register(&self, manifest: PluginManifest) -> Result<()> {
        if !allow_unverified_content_registration() {
            return Err(anyhow!(
                "Plugin '{}' v{} requires content verification; use register_bytes or register_file",
                manifest.id,
                manifest.version,
            ));
        }
        self.register_checked(manifest, None).await
    }

    /// Register a plugin after verifying its signature and byte content.
    pub async fn register_bytes(&self, manifest: PluginManifest, content: &[u8]) -> Result<()> {
        self.register_checked(manifest, Some(hash_bytes(content)))
            .await
    }

    /// Register a plugin after verifying its signature and file content.
    pub async fn register_file(&self, manifest: PluginManifest, path: &Path) -> Result<()> {
        self.register_checked(manifest, Some(hash_file(path)?))
            .await
    }

    async fn register_checked(
        &self,
        manifest: PluginManifest,
        actual_hash: Option<String>,
    ) -> Result<()> {
        if !self.signing_key.verify(&manifest) {
            return Err(anyhow!(
                "Plugin '{}' v{} has an invalid signature — refusing to register",
                manifest.id,
                manifest.version,
            ));
        }

        if let Some(actual_hash) = actual_hash {
            if actual_hash != manifest.content_hash {
                return Err(anyhow!(
                    "Plugin '{}' v{} content hash mismatch — refusing to register",
                    manifest.id,
                    manifest.version,
                ));
            }
        } else {
            tracing::warn!(
                plugin_id = %manifest.id,
                "Plugin registered through explicit unsafe content verification bypass"
            );
        }

        tracing::debug!(
            plugin_id = %manifest.id,
            manifest_hash = %manifest.content_hash,
            "Content hash verification completed"
        );

        tracing::info!(
            plugin_id = %manifest.id,
            version = %manifest.version,
            capabilities = ?manifest.capabilities,
            "Plugin registered and verified"
        );

        let mut plugins = self.plugins.write().await;
        plugins.insert(manifest.id.clone(), manifest);
        Ok(())
    }

    /// Check if a plugin is registered and verified.
    pub async fn is_verified(&self, plugin_id: &str) -> bool {
        self.plugins.read().await.contains_key(plugin_id)
    }

    /// Get a plugin manifest.
    pub async fn get(&self, plugin_id: &str) -> Option<PluginManifest> {
        self.plugins.read().await.get(plugin_id).cloned()
    }

    /// List all registered plugins.
    pub async fn list(&self) -> Vec<PluginManifest> {
        self.plugins.read().await.values().cloned().collect()
    }

    /// Get a reference to the signing key (for creating manifests).
    pub fn signing_key(&self) -> &SigningKey {
        &self.signing_key
    }

    /// Verify a plugin's content hash against a file on disk.
    pub async fn verify_content(&self, plugin_id: &str, path: &Path) -> Result<bool> {
        let manifest = self
            .get(plugin_id)
            .await
            .ok_or_else(|| anyhow!("Plugin '{}' not registered", plugin_id))?;
        let file_hash = hash_file(path)?;
        Ok(file_hash == manifest.content_hash)
    }
}

fn allow_unverified_content_registration() -> bool {
    std::env::var("CODETETHER_ALLOW_UNVERIFIED_PLUGIN_CONTENT")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Execute a tool in a sandboxed subprocess.
pub async fn execute_sandboxed(
    command: &str,
    args: &[String],
    policy: &SandboxPolicy,
    working_dir: Option<&Path>,
) -> Result<SandboxResult> {
    use std::time::Instant;
    use tokio::process::Command;

    let started = Instant::now();
    let mut violations = Vec::new();
    let mut unsafe_fallbacks = Vec::new();

    if !policy.allow_exec {
        return Err(anyhow!("Sandbox policy denies process execution"));
    }

    // Build restricted environment — strip everything except essentials.
    let mut env: HashMap<String, String> = HashMap::new();
    env.insert("PATH".to_string(), "/usr/bin:/bin".to_string());
    env.insert("HOME".to_string(), "/tmp".to_string());
    env.insert("LANG".to_string(), "C.UTF-8".to_string());
    env.insert("GIT_TERMINAL_PROMPT".to_string(), "0".to_string());
    env.insert("GCM_INTERACTIVE".to_string(), "never".to_string());
    env.insert("DEBIAN_FRONTEND".to_string(), "noninteractive".to_string());
    env.insert("SUDO_ASKPASS".to_string(), "/bin/false".to_string());
    env.insert("SSH_ASKPASS".to_string(), "/bin/false".to_string());
    inject_codetether_runtime_env(&mut env);

    unsafe_fallbacks.extend(sandbox_network::validate(policy, command, args)?);
    if !policy.allow_network {
        // Keep the advisory marker visible for cooperative child processes.
        // Non-cooperative known network commands are rejected before spawn.
        env.insert("CODETETHER_SANDBOX_NO_NETWORK".to_string(), "1".to_string());
    }

    let work_dir = working_dir
        .map(|p| p.to_path_buf())
        .unwrap_or_else(std::env::temp_dir);
    sandbox_paths::validate_working_dir(policy, &work_dir)?;

    let mut cmd = Command::new(command);
    cmd.args(args)
        .current_dir(&work_dir)
        .env_clear()
        .envs(&env)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());
    unsafe_fallbacks.extend(sandbox_limits::apply_memory_limit(
        &mut cmd,
        policy.max_memory_bytes,
    ));

    let timeout = std::time::Duration::from_secs(policy.timeout_secs);

    let child = cmd.spawn().context("Failed to spawn sandboxed process")?;

    let output = tokio::time::timeout(timeout, child.wait_with_output())
        .await
        .map_err(|_| {
            violations.push("timeout_exceeded".to_string());
            anyhow!("Sandboxed process timed out after {}s", policy.timeout_secs)
        })?
        .context("Failed to wait for sandboxed process")?;

    let duration_ms = started.elapsed().as_millis() as u64;
    let exit_code = output.status.code();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let combined_output = if stderr.is_empty() {
        stdout.to_string()
    } else {
        format!("{}\n--- stderr ---\n{}", stdout, stderr)
    };

    let output_hash = hash_bytes(combined_output.as_bytes());

    Ok(SandboxResult {
        tool_id: command.to_string(),
        success: output.status.success(),
        output: combined_output,
        output_hash,
        exit_code,
        duration_ms,
        sandbox_violations: violations,
        unsafe_fallbacks,
    })
}

fn inject_codetether_runtime_env(env: &mut HashMap<String, String>) {
    let Ok(current_exe) = std::env::current_exe() else {
        return;
    };
    env.insert(
        "CODETETHER_BIN".to_string(),
        current_exe.to_string_lossy().into_owned(),
    );

    let mut path_entries = current_exe
        .parent()
        .map(|parent| vec![parent.to_path_buf()])
        .unwrap_or_default();
    if let Some(existing_path) = env
        .get("PATH")
        .map(std::ffi::OsString::from)
        .or_else(|| std::env::var_os("PATH"))
    {
        path_entries.extend(std::env::split_paths(&existing_path));
    }
    if let Ok(path) = std::env::join_paths(path_entries) {
        env.insert("PATH".to_string(), path.to_string_lossy().into_owned());
    }
}

/// Constant-time byte comparison.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn signed_manifest(key: &SigningKey, id: &str, content: &[u8]) -> PluginManifest {
        let hash = hash_bytes(content);
        PluginManifest {
            id: id.to_string(),
            name: id.to_string(),
            version: "1.0.0".to_string(),
            content_hash: hash.clone(),
            signed_by: "test".to_string(),
            signature: key.sign(id, "1.0.0", &hash),
            capabilities: vec!["fs:read".to_string()],
            timeout_secs: 30,
        }
    }

    #[test]
    fn sign_and_verify_roundtrip() {
        let key = SigningKey::with_key(b"test-secret-key-for-signing".to_vec());
        let hash = hash_bytes(b"print('hello')");
        let sig = key.sign("my-plugin", "1.0.0", &hash);

        let manifest = PluginManifest {
            id: "my-plugin".to_string(),
            name: "My Plugin".to_string(),
            version: "1.0.0".to_string(),
            content_hash: hash,
            signed_by: "test".to_string(),
            signature: sig,
            capabilities: vec!["fs:read".to_string()],
            timeout_secs: 30,
        };

        assert!(key.verify(&manifest));
    }

    #[test]
    fn tampered_manifest_fails_verification() {
        let key = SigningKey::with_key(b"test-secret-key-for-signing".to_vec());
        let hash = hash_bytes(b"print('hello')");
        let sig = key.sign("my-plugin", "1.0.0", &hash);

        let manifest = PluginManifest {
            id: "my-plugin".to_string(),
            name: "My Plugin".to_string(),
            version: "1.0.1".to_string(), // tampered version
            content_hash: hash,
            signed_by: "test".to_string(),
            signature: sig,
            capabilities: vec![],
            timeout_secs: 30,
        };

        assert!(!key.verify(&manifest));
    }

    #[test]
    fn hash_bytes_is_deterministic() {
        let a = hash_bytes(b"hello world");
        let b = hash_bytes(b"hello world");
        assert_eq!(a, b);
        assert_ne!(a, hash_bytes(b"hello worl"));
    }

    #[tokio::test]
    async fn plugin_registry_rejects_bad_signature() {
        let key = SigningKey::with_key(b"test-key".to_vec());
        let registry = PluginRegistry::new(key);

        let manifest = PluginManifest {
            id: "bad-plugin".to_string(),
            name: "Bad".to_string(),
            version: "0.1.0".to_string(),
            content_hash: "abc".to_string(),
            signed_by: "attacker".to_string(),
            signature: "definitely-wrong".to_string(),
            capabilities: vec![],
            timeout_secs: 10,
        };

        assert!(registry.register_bytes(manifest, b"content").await.is_err());
        assert!(!registry.is_verified("bad-plugin").await);
    }

    #[tokio::test]
    async fn plugin_registry_registers_matching_content() {
        let key = SigningKey::with_key(b"test-key".to_vec());
        let manifest = signed_manifest(&key, "good-plugin", b"content");
        let registry = PluginRegistry::new(key);

        registry
            .register_bytes(manifest, b"content")
            .await
            .expect("matching content registers");
        assert!(registry.is_verified("good-plugin").await);
    }

    #[tokio::test]
    async fn plugin_registry_rejects_wrong_content_hash() {
        let key = SigningKey::with_key(b"test-key".to_vec());
        let manifest = signed_manifest(&key, "mismatch-plugin", b"content");
        let registry = PluginRegistry::new(key);

        assert!(
            registry
                .register_bytes(manifest, b"tampered")
                .await
                .is_err()
        );
        assert!(!registry.is_verified("mismatch-plugin").await);
    }

    #[tokio::test]
    async fn plugin_registry_requires_content_by_default() {
        let key = SigningKey::with_key(b"test-key".to_vec());
        let manifest = signed_manifest(&key, "needs-content", b"content");
        let registry = PluginRegistry::new(key);

        assert!(registry.register(manifest).await.is_err());
    }

    #[tokio::test]
    async fn sandbox_denies_working_dir_outside_allowed_paths() {
        let allowed = tempfile::tempdir().expect("allowed tempdir");
        let denied = tempfile::tempdir().expect("denied tempdir");
        let policy = SandboxPolicy {
            allowed_paths: vec![allowed.path().to_path_buf()],
            allow_network: true,
            allow_exec: true,
            ..SandboxPolicy::default()
        };

        let err = execute_sandboxed(
            "sh",
            &["-c".to_string(), "echo denied".to_string()],
            &policy,
            Some(denied.path()),
        )
        .await
        .expect_err("denied path should fail closed");
        assert!(err.to_string().contains("denied working directory"));
    }

    #[tokio::test]
    async fn sandbox_denies_network_commands_when_network_disabled() {
        let policy = SandboxPolicy {
            allow_network: false,
            allow_exec: true,
            ..SandboxPolicy::default()
        };
        let err = execute_sandboxed(
            "sh",
            &["-c".to_string(), "curl https://example.com".to_string()],
            &policy,
            None,
        )
        .await
        .expect_err("network command should fail closed");
        assert!(err.to_string().contains("denies network access"));
    }
}
