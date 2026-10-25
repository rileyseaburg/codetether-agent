use super::{
    cookies::{Cookie, Jar, now},
    rotation::Rotation,
};
use reqwest::Url;
impl Jar {
    pub fn header(&self, url: &Url, updates: &[Rotation]) -> String {
        let mut cookies = self.0.clone();
        for update in updates {
            cookies
                .retain(|c| !(c.domain == "google.com" && c.path == "/" && c.name == update.name));
            if update.expires > now() && !update.value.is_empty() {
                cookies.push(Cookie {
                    domain: "google.com".into(),
                    subdomains: true,
                    path: "/".into(),
                    secure: true,
                    expires: update.expires,
                    name: update.name.clone(),
                    value: update.value.clone(),
                });
            }
        }
        let host = url.host_str().unwrap_or_default();
        cookies.retain(|c| {
            (c.expires == 0 || c.expires > now())
                && (!c.secure || url.scheme() == "https")
                && (host == c.domain || (c.subdomains && host.ends_with(&format!(".{}", c.domain))))
                && (url.path() == c.path
                    || url
                        .path()
                        .strip_prefix(&c.path)
                        .is_some_and(|tail| c.path.ends_with('/') || tail.starts_with('/')))
        });
        cookies.sort_by_key(|c| std::cmp::Reverse(c.path.len()));
        cookies
            .iter()
            .map(|c| format!("{}={}", c.name, c.value))
            .collect::<Vec<_>>()
            .join("; ")
    }
}
