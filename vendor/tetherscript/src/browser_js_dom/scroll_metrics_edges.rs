use super::super::*;

pub(super) fn border(handle: &DomHandle) -> [i64; 4] {
    let Some(styled) = styled_node_at_path(handle) else {
        return [0; 4];
    };
    let mut edges = box_values(styled.styles.get("border-width")).unwrap_or([0; 4]);
    edges[0] = edge(&styled.styles, "border-width-top", "border-top-width", edges[0]);
    edges[1] = edge(
        &styled.styles,
        "border-width-right",
        "border-right-width",
        edges[1],
    );
    edges[2] = edge(
        &styled.styles,
        "border-width-bottom",
        "border-bottom-width",
        edges[2],
    );
    edges[3] = edge(
        &styled.styles,
        "border-width-left",
        "border-left-width",
        edges[3],
    );
    edges
}

fn edge(styles: &HashMap<String, String>, legacy: &str, standard: &str, fallback: i64) -> i64 {
    styles
        .get(legacy)
        .and_then(|value| part(value))
        .or_else(|| styles.get(standard).and_then(|value| part(value)))
        .unwrap_or(fallback)
        .max(0)
}

fn box_values(value: Option<&String>) -> Option<[i64; 4]> {
    let values = value?.split_whitespace().filter_map(part).collect::<Vec<_>>();
    match values.as_slice() {
        [all] => Some([*all, *all, *all, *all]),
        [vertical, horizontal] => Some([*vertical, *horizontal, *vertical, *horizontal]),
        [top, horizontal, bottom] => Some([*top, *horizontal, *bottom, *horizontal]),
        [top, right, bottom, left, ..] => Some([*top, *right, *bottom, *left]),
        _ => None,
    }
}

fn part(value: &str) -> Option<i64> {
    let value = value.trim();
    value.strip_suffix("px").unwrap_or(value).trim().parse().ok()
}
