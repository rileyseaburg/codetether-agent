//! Form purpose classification.

mod types;

use super::InputSummary;
pub use types::FormPurpose;

fn hay(inputs: &[InputSummary]) -> String {
    inputs
        .iter()
        .map(|i| {
            format!(
                "{} {} {} {} {}",
                i.input_type,
                i.name.clone().unwrap_or_default(),
                i.id.clone().unwrap_or_default(),
                i.placeholder.clone().unwrap_or_default(),
                i.label.clone().unwrap_or_default()
            )
        })
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}

fn contains_any(s: &str, xs: &[&str]) -> bool {
    xs.iter().any(|x| s.contains(x))
}

pub fn classify_form(inputs: &[InputSummary]) -> FormPurpose {
    let h = hay(inputs);
    let has_pw = inputs
        .iter()
        .any(|i| i.input_type.eq_ignore_ascii_case("password"));
    if contains_any(&h, &["search", "query", "q"]) && !has_pw {
        return FormPurpose::Search;
    }
    if has_pw && contains_any(&h, &["confirm", "sign up", "register", "create"]) {
        return FormPurpose::Registration;
    }
    if has_pw {
        return FormPurpose::Login;
    }
    if contains_any(&h, &["card", "checkout", "billing", "shipping"]) {
        return FormPurpose::Checkout;
    }
    if contains_any(&h, &["newsletter", "subscribe"]) {
        return FormPurpose::Newsletter;
    }
    if contains_any(&h, &["message", "contact", "subject"]) {
        return FormPurpose::Contact;
    }
    FormPurpose::Other
}
