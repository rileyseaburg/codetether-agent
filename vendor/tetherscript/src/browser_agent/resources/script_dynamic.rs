//! Dynamic import rewriting for deterministic module resources.

use std::collections::HashSet;

use super::{script_dynamic_ref, script_namespace, script_resolve, url, ResourceRegistry};

pub(crate) fn rewrite(
    registry: &ResourceRegistry,
    current_url: &str,
    body: &str,
    seen: &mut HashSet<String>,
    out: &mut String,
) -> Result<String, String> {
    let refs = script_dynamic_ref::collect(body);
    let mut rewritten = String::new();
    let mut cursor = 0;
    for item in refs {
        let (url, source) = script_resolve::text(registry, current_url, &item.url)
            .ok_or_else(|| script_resolve::missing(current_url, &item.url))?;
        let module_url = url::resolve(current_url, &url);
        out.push_str(&super::script_module::expand(
            registry,
            &module_url,
            source,
            seen,
        )?);
        out.push_str(&script_namespace::binding(&module_url, source));
        rewritten.push_str(&body[cursor..item.start]);
        rewritten.push_str("Promise.resolve(");
        rewritten.push_str(&script_namespace::name(&module_url));
        rewritten.push(')');
        cursor = item.end;
    }
    rewritten.push_str(&body[cursor..]);
    Ok(rewritten)
}
