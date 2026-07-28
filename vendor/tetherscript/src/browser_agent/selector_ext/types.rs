//! Parsed selector extension types.

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SelectorFilter {
    Visible,
    Enabled,
    Disabled,
    Checked,
    HasText(String),
    Nth(usize),
    Invalid,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SelectorPlan {
    pub base: String,
    pub filters: Vec<SelectorFilter>,
}

impl SelectorPlan {
    pub(crate) fn new(base: String, filters: Vec<SelectorFilter>) -> Self {
        Self { base, filters }
    }

    pub(crate) fn invalid(&self) -> bool {
        self.filters
            .iter()
            .any(|filter| matches!(filter, SelectorFilter::Invalid))
    }

    pub(crate) fn nth(&self) -> Option<usize> {
        self.filters.iter().find_map(|filter| match filter {
            SelectorFilter::Nth(index) => Some(*index),
            _ => None,
        })
    }
}

pub(crate) fn normalized_base(raw: String) -> String {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        "*".into()
    } else {
        trimmed.into()
    }
}
