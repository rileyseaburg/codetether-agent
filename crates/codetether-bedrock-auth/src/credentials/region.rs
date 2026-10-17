//! Region lookup from environment and shared config.
use crate::*;
impl AwsCredentials {
    /// Detect the target AWS region.
    ///
    /// Precedence: `AWS_REGION` → `AWS_DEFAULT_REGION` → `~/.aws/config`
    /// (default or `[profile NAME]` per `AWS_PROFILE`).
    ///
    /// # Returns
    ///
    /// `Some(region)` on success, `None` if no region could be determined.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use codetether_bedrock_auth::AwsCredentials;
    ///
    /// let region = AwsCredentials::detect_region()
    ///     .unwrap_or_else(|| "us-east-1".to_string());
    /// assert!(!region.is_empty());
    /// ```
    pub fn detect_region() -> Option<String> {
        if let Ok(r) = std::env::var("AWS_REGION")
            && !r.is_empty()
        {
            return Some(r);
        }
        if let Ok(r) = std::env::var("AWS_DEFAULT_REGION")
            && !r.is_empty()
        {
            return Some(r);
        }
        let profile = std::env::var("AWS_PROFILE").unwrap_or_else(|_| "default".to_string());
        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .ok()?;
        let path = std::path::Path::new(&home).join(".aws").join("config");
        let content = std::fs::read_to_string(&path).ok()?;

        let section_header = if profile == "default" {
            "[default]".to_string()
        } else {
            format!("[profile {profile}]")
        };
        let mut in_section = false;
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with('[') {
                in_section = trimmed == section_header;
                continue;
            }
            if !in_section {
                continue;
            }
            if let Some((k, v)) = trimmed.split_once('=')
                && k.trim() == "region"
            {
                let v = v.trim();
                if !v.is_empty() {
                    return Some(v.to_string());
                }
            }
        }
        None
    }
}
