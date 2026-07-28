use super::super::*;

pub(super) fn selected_index(select: &DomHandle) -> isize {
    let options = super::handles::all(select);
    if options.is_empty() || super::state::is_none(select) {
        return -1;
    }
    options.iter().position(super::value::selected).unwrap_or(0) as isize
}

pub(super) fn value(select: &DomHandle) -> String {
    let index = selected_index(select);
    if index < 0 {
        return String::new();
    }
    super::handles::all(select)
        .get(index as usize)
        .map(super::value::get)
        .unwrap_or_default()
}
