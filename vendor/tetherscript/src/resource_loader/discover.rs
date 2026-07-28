//! Resource discovery from DOM/HTML.
use super::{CorsMode, CredentialsMode, ResourcePriority as P, ResourceRequest, ResourceType as T};

/// Discover external resources by scanning an HTML string.
pub fn discover_resources<D: core::fmt::Debug>(
    document: &D,
    base_url: &str,
) -> Vec<ResourceRequest> {
    let html = format!("{:?}", document);
    let mut out = Vec::new();
    for tag in tags(&html) {
        let name = tag_name(&tag);
        let rel = attr(&tag, "rel").unwrap_or_default().to_ascii_lowercase();
        let src = attr(&tag, "src");
        let href = attr(&tag, "href");
        let (url, kind, pri) = match name.as_str() {
            "link" if rel.contains("stylesheet") => (href, T::Stylesheet, P::Css),
            "link" if rel.contains("preload") => (href, T::Preload, P::Preload),
            "link" if rel.contains("prefetch") => (href, T::Prefetch, P::Prefetch),
            "link" if rel.contains("dns-prefetch") => (href, T::DnsPrefetch, P::Prefetch),
            "link" if rel.contains("preconnect") => (href, T::Preconnect, P::Prefetch),
            "script" => (
                src,
                T::Script,
                if has_attr(&tag, "async") || has_attr(&tag, "defer") {
                    P::AsyncScript
                } else {
                    P::HeadScript
                },
            ),
            "img" => (src, T::Image, P::Image),
            "video" => (src, T::Video, P::Media),
            "audio" => (src, T::Audio, P::Media),
            "source" => (src, T::Source, P::Media),
            "iframe" => (src, T::Iframe, P::Frame),
            _ => (None, T::Other, P::Prefetch),
        };
        if let Some(u) = url {
            let mut r = ResourceRequest::new(resolve(base_url, &u), kind, pri, name);
            r.integrity = attr(&tag, "integrity");
            if attr(&tag, "crossorigin").is_some() {
                r.cors_mode = CorsMode::Cors;
            }
            if attr(&tag, "crossorigin").as_deref() == Some("use-credentials") {
                r.credentials_mode = CredentialsMode::Include;
            }
            out.push(r);
        }
    }
    out
}

fn tags(s: &str) -> Vec<String> {
    let mut v = Vec::new();
    let mut r = s;
    while let Some(i) = r.find('<') {
        r = &r[i + 1..];
        if let Some(j) = r.find('>') {
            v.push(r[..j].to_string());
            r = &r[j + 1..];
        } else {
            break;
        }
    }
    v
}

fn tag_name(t: &str) -> String {
    t.trim_start_matches('/')
        .split_whitespace()
        .next()
        .unwrap_or("")
        .trim_matches('/')
        .to_ascii_lowercase()
}

fn has_attr(t: &str, n: &str) -> bool {
    attr(t, n).is_some() || t.to_ascii_lowercase().contains(&format!(" {}", n))
}

fn attr(t: &str, n: &str) -> Option<String> {
    let l = t.to_ascii_lowercase();
    let p = format!("{}=", n);
    let i = l.find(&p)? + p.len();
    let b = t.as_bytes().get(i).copied()?;
    let q = b as char;
    if q == '\'' || q == '"' {
        let e = t[i + 1..].find(q)?;
        Some(t[i + 1..i + 1 + e].into())
    } else {
        Some(t[i..].split_whitespace().next()?.trim_matches('/').into())
    }
}

fn resolve(base: &str, u: &str) -> String {
    if u.contains("://") || u.starts_with("data:") {
        u.into()
    } else if u.starts_with('/') {
        format!("{}{}", super::scheduler::origin_of(base), u)
    } else {
        format!("{}/{}", base.trim_end_matches('/'), u)
    }
}
