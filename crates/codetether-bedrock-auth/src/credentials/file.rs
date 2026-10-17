//! Named shared-credential file parsing.
use crate::*;
impl AwsCredentials {
    /// Parse `~/.aws/credentials` INI file for the given profile.
    ///
    /// # Arguments
    ///
    /// * `profile` — Profile name (e.g. `"default"`, `"prod"`).
    ///
    /// # Returns
    ///
    /// `Some(creds)` if a matching `[profile]` section with both
    /// `aws_access_key_id` and `aws_secret_access_key` was found.
    pub(super) fn from_credentials_file(profile: &str) -> Option<Self> {
        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .ok()?;
        let path = std::path::Path::new(&home).join(".aws").join("credentials");
        let content = std::fs::read_to_string(&path).ok()?;

        let section_header = format!("[{profile}]");
        let mut in_section = false;
        let mut key_id = None;
        let mut secret = None;
        let mut token = None;

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with('[') {
                in_section = trimmed == section_header;
                continue;
            }
            if !in_section {
                continue;
            }
            if let Some((k, v)) = trimmed.split_once('=') {
                let k = k.trim();
                let v = v.trim();
                match k {
                    "aws_access_key_id" => key_id = Some(v.to_string()),
                    "aws_secret_access_key" => secret = Some(v.to_string()),
                    "aws_session_token" => token = Some(v.to_string()),
                    _ => {}
                }
            }
        }

        Some(Self {
            access_key_id: key_id?,
            secret_access_key: secret?,
            session_token: token,
        })
    }
}
