use super::model::Sheet;

pub(super) fn css(sheets: &[Sheet]) -> String {
    sheets
        .iter()
        .map(sheet_css)
        .filter(|source| !source.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn sheet_css(sheet: &Sheet) -> String {
    sheet
        .rules
        .iter()
        .map(|rule| rule.css_text.clone())
        .collect::<Vec<_>>()
        .join("\n")
}
