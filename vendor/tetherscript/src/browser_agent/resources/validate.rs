//! External resource validation API for browser pages.

use crate::browser_agent::page::BrowserPage;

use super::{discover, preload, url, validate_modules, ResourceKind};

impl BrowserPage {
    /// Validate that every external page resource has been registered.
    ///
    /// This checks executable scripts, stylesheets, images, and passive preload
    /// links such as `rel="modulepreload"` without performing ambient network
    /// I/O or executing scripts.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` when each referenced resource can be resolved from the
    /// page-local resource registry.
    ///
    /// # Errors
    ///
    /// Returns an error naming each missing resource reference and its resolved
    /// URL candidate.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tetherscript::browser_agent::BrowserPage;
    ///
    /// let mut page = BrowserPage::from_html(
    ///     "https://app.test/index.html",
    ///     "<script src='/app.js'></script>",
    /// );
    /// page.register_script_resource("/app.js", "window.ready = true;");
    ///
    /// assert!(page.validate_external_resources().is_ok());
    /// ```
    pub fn validate_external_resources(&self) -> Result<(), String> {
        let missing = missing_resources(self);
        if missing.is_empty() {
            Ok(())
        } else {
            Err(format!(
                "missing external resources: {}",
                missing.join(", ")
            ))
        }
    }
}

fn missing_resources(page: &BrowserPage) -> Vec<String> {
    let mut refs = discover::collect(&page.session.document);
    refs.extend(preload::collect(&page.session.document));
    let mut missing = refs
        .iter()
        .filter_map(|reference| missing_resource(page, reference))
        .collect::<Vec<_>>();
    missing.extend(validate_modules::missing(page));
    missing
}

fn missing_resource(page: &BrowserPage, reference: &discover::ResourceReference) -> Option<String> {
    if url::candidates(&page.session.url, &reference.url)
        .iter()
        .any(|candidate| page.resources.has(reference.kind, candidate))
    {
        return None;
    }
    Some(format!(
        "{} {} (resolved {})",
        kind_name(reference.kind),
        reference.url,
        url::resolve(&page.session.url, &reference.url)
    ))
}

fn kind_name(kind: ResourceKind) -> &'static str {
    match kind {
        ResourceKind::Script => "script",
        ResourceKind::Stylesheet => "stylesheet",
        ResourceKind::Image => "image",
        ResourceKind::SourceMap => "source map",
    }
}
