use super::authority::HttpAuthority;
use super::url::ParsedUrl;

impl HttpAuthority {
    pub(super) fn check_scope(&self, method: &str, url: &str) -> Result<(), String> {
        if !self.methods.contains(method) {
            return Err(format!("http: method {} not allowed by capability", method));
        }
        let parsed = ParsedUrl::parse(url)?;
        let origin = parsed.origin();
        if !self.origins.iter().any(|allowed| allowed == &origin) {
            return Err(format!(
                "http: origin {} is not in the allowed set ({:?})",
                origin, self.origins
            ));
        }
        if let Some(prefix) = &self.path_prefix {
            if !parsed.path.starts_with(prefix) {
                return Err(format!(
                    "http: path {} does not match required prefix {}",
                    parsed.path, prefix
                ));
            }
        }
        Ok(())
    }
}
