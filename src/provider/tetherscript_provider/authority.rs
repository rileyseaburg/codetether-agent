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
    ProviderAuthority::with_bound_header(
        ProviderAuthority::new(base_url),
        "Authorization",
        &format!("Bearer {api_key}"),
    )
}
