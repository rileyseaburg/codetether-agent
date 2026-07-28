//! Source-map segment selection.

use super::super::mapped_types::OriginalSourceLocation;
use super::parsed::ParsedSourceMap;

#[derive(Default)]
struct State(i64, i64, i64, i64);

pub fn original(
    map: &ParsedSourceMap,
    line: usize,
    column: usize,
) -> Option<OriginalSourceLocation> {
    let mut state = State::default();
    let mut best = None;
    let target_line = line.checked_sub(1)?;
    let target_column = column.checked_sub(1)? as i64;
    for (row_index, row) in map.mappings.split(';').enumerate() {
        state.0 = 0;
        for segment in row.split(',').filter(|item| !item.is_empty()) {
            apply(&mut state, &super::vlq::decode(segment)?)?;
            if row_index == target_line && state.0 <= target_column {
                best = location(map, &state);
            }
        }
        if row_index >= target_line {
            break;
        }
    }
    best
}

fn apply(state: &mut State, values: &[i64]) -> Option<()> {
    state.0 += *values.first()?;
    if values.len() >= 4 {
        state.1 += values[1];
        state.2 += values[2];
        state.3 += values[3];
    }
    Some(())
}

fn location(map: &ParsedSourceMap, state: &State) -> Option<OriginalSourceLocation> {
    let source = map.sources.get(usize::try_from(state.1).ok()?)?;
    Some(OriginalSourceLocation {
        source_url: format!("{}{}", map.source_root, source),
        line: usize::try_from(state.2).ok()?.saturating_add(1),
        column: usize::try_from(state.3).ok()?.saturating_add(1),
    })
}
