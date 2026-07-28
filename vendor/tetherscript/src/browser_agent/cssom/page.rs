//! Browser page APIs for computed-style inspection.

use crate::browser_agent::locator::Locator;
use crate::browser_agent::page::BrowserPage;
use crate::browser_agent::query::{self, DomMatch};

use super::{active_css, lookup, ComputedStyle};

impl BrowserPage {
    /// Return deterministic computed style for one located element.
    ///
    /// # Arguments
    ///
    /// * `locator` - Element locator. Strict locators must match exactly once.
    ///
    /// # Returns
    ///
    /// A stable computed-style property snapshot.
    ///
    /// # Errors
    ///
    /// Returns `Err` when resources cannot be prepared or the locator does not
    /// resolve according to its strictness.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tetherscript::browser_agent::{BrowserPage, Locator};
    ///
    /// let mut page = BrowserPage::from_html("mem://style", "<main id='x' style='color:red'></main>");
    /// let style = page.computed_style(&Locator::css("#x")).unwrap();
    /// assert_eq!(style.get("color"), Some("red"));
    /// ```
    pub fn computed_style(&mut self, locator: &Locator) -> Result<ComputedStyle, String> {
        self.prepare_external_resources()?;
        let dom = select(locator, query::locate(&self.session.document, locator))?;
        let css = active_css::active_css(&self.session.css, self.viewport_width, self.media);
        lookup::at_path(&self.session.document, &css, &dom.path)
            .ok_or_else(|| format!("locator {:?} has no computed style", locator.kind))
    }

    /// Return one computed CSS property for a located element.
    ///
    /// # Arguments
    ///
    /// * `locator` - Element locator. Strict locators must match exactly once.
    /// * `name` - CSS property name, matched case-insensitively.
    ///
    /// # Returns
    ///
    /// The computed property value, or `None` when no value is present.
    ///
    /// # Errors
    ///
    /// Returns `Err` when external resources cannot be prepared or the locator
    /// does not resolve according to its strictness.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tetherscript::browser_agent::{BrowserPage, Locator};
    ///
    /// let mut page = BrowserPage::from_html("mem://style", "<main id='x'></main>");
    /// assert_eq!(
    ///     page.style_property(&Locator::css("#x"), "display").unwrap(),
    ///     Some("block".into())
    /// );
    /// ```
    pub fn style_property(
        &mut self,
        locator: &Locator,
        name: &str,
    ) -> Result<Option<String>, String> {
        Ok(self.computed_style(locator)?.property(name))
    }
}

fn select(locator: &Locator, matches: Vec<DomMatch>) -> Result<DomMatch, String> {
    if matches.is_empty() {
        return Err(format!("locator {:?} matched no elements", locator.kind));
    }
    if locator.strict && matches.len() != 1 {
        return Err(format!(
            "locator {:?} matched {} elements",
            locator.kind,
            matches.len()
        ));
    }
    Ok(matches[0].clone())
}
