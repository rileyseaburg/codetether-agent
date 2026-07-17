use std::rc::Rc;

use tetherscript::capability::Authority;
use tetherscript::plugin::{PluginHost, TetherScriptAuthority};
use tetherscript::provider_cap::ProviderAuthority;

pub fn host(base_url: &str, api_key: &str) -> PluginHost {
    let mut host = PluginHost::new();
    host.grant("tetherscript", TetherScriptAuthority::new());
    host.grant("provider", provider_authority(base_url, api_key));
    host
}

fn provider_authority(base_url: &str, api_key: &str) -> Rc<dyn Authority> {
    let auth = ProviderAuthority::new(base_url);
    let auth = ProviderAuthority::with_path(auth, &chat_completions_path(base_url));
    ProviderAuthority::with_bound_header(auth, "Authorization", &format!("Bearer {api_key}"))
}

fn chat_completions_path(base_url: &str) -> String {
    let without_scheme = base_url
        .strip_prefix("https://")
        .or_else(|| base_url.strip_prefix("http://"))
        .unwrap_or(base_url);
    let path = without_scheme
        .split_once('/')
        .map(|(_, path)| format!("/{path}"))
        .unwrap_or_else(|| "/v1".to_string());
    let path = path.trim_end_matches('/');
    if path.ends_with("/chat/completions") {
        path.to_string()
    } else {
        format!("{path}/chat/completions")
    }
}

#[cfg(test)]
mod tests {
    use super::chat_completions_path;

    #[test]
    fn derives_chat_completions_path_from_base_url() {
        assert_eq!(
            chat_completions_path("https://api.gsa.usai.gov/api/v1"),
            "/api/v1/chat/completions"
        );
        assert_eq!(
            chat_completions_path("http://localhost:8080/v1/"),
            "/v1/chat/completions"
        );
        assert_eq!(
            chat_completions_path("https://example.com/api/v1/chat/completions"),
            "/api/v1/chat/completions"
        );
    }
}
