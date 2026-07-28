//! Public page API for deterministic source maps.

use crate::browser_agent::page::BrowserPage;

use super::ResourceKind;

impl BrowserPage {
    /// Register source-map text for a bundled JavaScript resource.
    ///
    /// # Arguments
    ///
    /// * `url` - Source-map URL from a `sourceMappingURL` comment.
    /// * `source` - Raw source-map JSON text.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tetherscript::browser_agent::BrowserPage;
    ///
    /// let mut page = BrowserPage::from_html("mem://app", "");
    /// page.register_source_map_resource("/app.js.map", "{}");
    /// ```
    pub fn register_source_map_resource(
        &mut self,
        url: impl Into<String>,
        source: impl Into<String>,
    ) {
        self.resources
            .register_text(url, ResourceKind::SourceMap, source);
    }
}
