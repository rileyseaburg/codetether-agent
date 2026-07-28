#[derive(Clone)]
pub(super) struct Declaration {
    pub name: String,
    pub value: String,
}

#[derive(Clone)]
pub(super) struct Rule {
    pub selector_text: String,
    pub declarations: Vec<Declaration>,
    pub css_text: String,
}

#[derive(Clone)]
pub(super) struct Sheet {
    pub rules: Vec<Rule>,
}
