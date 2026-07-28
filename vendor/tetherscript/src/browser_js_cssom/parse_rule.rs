use crate::browser;

use super::model::Rule;
use super::parse_decl;

pub(super) fn rules(source: &str) -> Vec<Rule> {
    source
        .split('}')
        .filter_map(|part| {
            let part = part.trim();
            (!part.is_empty())
                .then(|| format!("{part}}}"))
                .and_then(|rule| parse(&rule).ok())
        })
        .collect()
}

pub(super) fn parse(source: &str) -> Result<Rule, String> {
    let source = source.trim();
    let open = source
        .find('{')
        .ok_or_else(|| "insertRule: expected '{' in CSS rule".to_string())?;
    let close = source
        .rfind('}')
        .ok_or_else(|| "insertRule: expected '}' in CSS rule".to_string())?;
    if close <= open {
        return Err("insertRule: rule body is empty".into());
    }
    let selector_text = source[..open].trim().to_string();
    let declarations = parse_decl::parse(&source[open + 1..close]);
    if selector_text.is_empty() || declarations.is_empty() {
        return Err("insertRule: expected selector and declarations".into());
    }
    let css_text = format!(
        "{} {{ {} }}",
        selector_text,
        parse_decl::css_text(&declarations)
    );
    if browser::parse_css(&css_text).is_empty() {
        return Err(format!(
            "insertRule: unsupported selector `{selector_text}`"
        ));
    }
    Ok(Rule {
        selector_text,
        declarations,
        css_text,
    })
}
