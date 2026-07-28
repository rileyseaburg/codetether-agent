//! Public page APIs for deterministic external resources.

use crate::browser_agent::page::BrowserPage;

use super::{BrowserResource, ImageResourceMetadata, ResourceKind};

impl BrowserPage {
    /// Register JavaScript source for a `<script src>` URL.
    ///
    /// # Arguments
    ///
    /// * `url` - Resource URL matched against script `src` attributes.
    /// * `source` - JavaScript source to execute during [`BrowserPage::run_scripts`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tetherscript::browser_agent::BrowserPage;
    ///
    /// let mut page = BrowserPage::from_html("mem://page", "<script src='/app.js'></script>");
    /// page.register_script_resource("/app.js", "window.ready = true;");
    /// ```
    pub fn register_script_resource(&mut self, url: impl Into<String>, source: impl Into<String>) {
        self.resources
            .register_text(url, ResourceKind::Script, source);
        self.runtime = None;
    }

    /// Register CSS source for a stylesheet link URL.
    ///
    /// # Arguments
    ///
    /// * `url` - Resource URL matched against stylesheet `href` attributes.
    /// * `source` - CSS source appended to the page stylesheet set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tetherscript::browser_agent::BrowserPage;
    ///
    /// let mut page = BrowserPage::from_html("mem://page", "<link rel='stylesheet' href='/app.css'>");
    /// page.register_stylesheet_resource("/app.css", "main { width: 4px; }");
    /// ```
    pub fn register_stylesheet_resource(
        &mut self,
        url: impl Into<String>,
        source: impl Into<String>,
    ) {
        self.resources
            .register_text(url, ResourceKind::Stylesheet, source);
        self.runtime = None;
    }

    /// Register image bytes for later agent inspection.
    ///
    /// # Arguments
    ///
    /// * `url` - Resource URL associated with image elements.
    /// * `bytes` - Raw deterministic image bytes stored for inspection.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tetherscript::browser_agent::BrowserPage;
    ///
    /// let mut page = BrowserPage::from_html("mem://page", "<img src='/logo.png'>");
    /// page.register_image_resource("/logo.png", vec![1, 2, 3]);
    /// ```
    pub fn register_image_resource(&mut self, url: impl Into<String>, bytes: Vec<u8>) {
        self.resources.register_image(url, bytes);
    }

    /// Return registered resources in deterministic insertion order.
    ///
    /// # Returns
    ///
    /// A cloned list of resources registered on this page.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tetherscript::browser_agent::BrowserPage;
    ///
    /// let mut page = BrowserPage::new(Default::default());
    /// page.register_script_resource("/app.js", "");
    /// assert_eq!(page.resources().len(), 1);
    /// ```
    pub fn resources(&self) -> Vec<BrowserResource> {
        self.resources.entries().to_vec()
    }

    /// Return byte-count metadata for registered image resources.
    ///
    /// # Returns
    ///
    /// Metadata for image resources registered on this page.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tetherscript::browser_agent::BrowserPage;
    ///
    /// let mut page = BrowserPage::new(Default::default());
    /// page.register_image_resource("/logo.png", vec![1, 2]);
    /// assert_eq!(page.image_resource_metadata()[0].byte_len, 2);
    /// ```
    pub fn image_resource_metadata(&self) -> Vec<ImageResourceMetadata> {
        self.resources.image_metadata()
    }
}
