//! Link classification.

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LinkKind {
    Navigation,
    Action,
    External,
    Download,
    Anchor,
}

pub fn classify_link(href: &str, text: &str, context: &str) -> LinkKind {
    let h = href.to_ascii_lowercase();
    let t = text.to_ascii_lowercase();
    let c = context.to_ascii_lowercase();
    if h.starts_with('#') {
        return LinkKind::Anchor;
    }
    if ["download", ".pdf", ".zip", ".csv"]
        .iter()
        .any(|x| h.contains(x) || t.contains(x))
    {
        return LinkKind::Download;
    }
    if h.starts_with("http://") || h.starts_with("https://") {
        return LinkKind::External;
    }
    if h.starts_with("javascript:") || ["buy", "submit", "add"].iter().any(|x| t.contains(x)) {
        return LinkKind::Action;
    }
    if ["nav", "menu", "header", "footer"]
        .iter()
        .any(|x| c.contains(x))
    {
        return LinkKind::Navigation;
    }
    LinkKind::Navigation
}
