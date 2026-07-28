//! Page summary data types.

use super::super::actions::ActionableElement;
use super::super::forms::FormPurpose;
use super::super::links::LinkKind;
use super::super::regions::RegionInfo;

#[derive(Clone, Debug)]
pub struct LinkInfo {
    pub href: String,
    pub text: String,
    pub kind: LinkKind,
}

impl LinkInfo {
    pub fn new(href: &str, text: &str, kind: LinkKind) -> Self {
        Self {
            href: href.into(),
            text: text.into(),
            kind,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FormInfo {
    pub purpose: FormPurpose,
    pub input_count: usize,
}

impl FormInfo {
    pub fn new(purpose: FormPurpose, input_count: usize) -> Self {
        Self {
            purpose,
            input_count,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct PageSummary {
    pub regions: Vec<RegionInfo>,
    pub forms: Vec<FormInfo>,
    pub links: Vec<LinkInfo>,
    pub actions: Vec<ActionableElement>,
}
