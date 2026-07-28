//! Page integration for deterministic external resources.

use crate::browser::parse_html;
use crate::browser_agent::page::BrowserPage;

use super::{discover, script, style};

impl BrowserPage {
    pub(crate) fn prepare_external_resources(&mut self) -> Result<(), String> {
        let refs = discover::collect(&self.session.document);
        if refs.is_empty() {
            return Ok(());
        }
        self.apply_external_styles(&refs)?;
        self.inline_external_scripts(&refs)
    }

    fn apply_external_styles(
        &mut self,
        refs: &[discover::ResourceReference],
    ) -> Result<(), String> {
        self.session.css = style::append_stylesheets(
            self.session.css.clone(),
            refs,
            &self.resources,
            &self.session.url,
        )?;
        Ok(())
    }

    fn inline_external_scripts(
        &mut self,
        refs: &[discover::ResourceReference],
    ) -> Result<(), String> {
        if !refs
            .iter()
            .any(|item| item.kind == super::ResourceKind::Script)
        {
            return Ok(());
        }
        if let Some(html) =
            script::inline_scripts(&self.session.document, &self.resources, &self.session.url)?
        {
            self.session.html = html;
            self.session.document = parse_html(&self.session.html);
            self.runtime = None;
        }
        Ok(())
    }
}
