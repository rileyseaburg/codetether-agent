//! Page understanding: region detection, form classification, link classification, actionable elements.

pub mod actions;
pub mod forms;
pub mod links;
pub mod regions;
pub mod summary;

pub use actions::ActionableElement;
pub use forms::FormPurpose;
pub use links::LinkKind;
pub use regions::{PageRegion, RegionInfo};
pub use summary::PageSummary;

#[derive(Clone, Debug, Default)]
pub struct ElementSummary {
    pub selector: String,
    pub tag: String,
    pub role: Option<String>,
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub text: String,
    pub attrs: Vec<(String, String)>,
}

impl ElementSummary {
    pub fn attr(&self, name: &str) -> Option<&str> {
        self.attrs
            .iter()
            .find(|(k, _)| k == name)
            .map(|(_, v)| v.as_str())
    }
}

#[derive(Clone, Debug, Default)]
pub struct InputSummary {
    pub selector: String,
    pub input_type: String,
    pub name: Option<String>,
    pub id: Option<String>,
    pub placeholder: Option<String>,
    pub label: Option<String>,
}

#[cfg(test)]
mod tests;
