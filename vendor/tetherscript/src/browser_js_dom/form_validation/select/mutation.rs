use super::super::*;

pub(super) fn set_index(select: &DomHandle, index: isize) {
    let options = super::handles::all(select);
    if index < 0 || index as usize >= options.len() {
        for option in options {
            super::sync::selected_attr(&option, false);
        }
        super::state::set_none(select);
    } else {
        super::state::clear(select);
        for (option_index, option) in options.into_iter().enumerate() {
            super::sync::selected_attr(&option, option_index == index as usize);
        }
    }
    super::sync::value_attr(select);
    super::objects::refresh(select);
}

pub(super) fn set_value(select: &DomHandle, value: &str) {
    let options = super::handles::all(select);
    if let Some(index) = options
        .iter()
        .position(|option| super::value::get(option) == value)
    {
        set_index(select, index as isize);
    }
}

pub(super) fn set_option_selected(option: &DomHandle, selected: bool) {
    if let Some(select) = super::owner::select(option) {
        if selected && !multiple(&select) {
            set_index(&select, super::owner::index(option));
            return;
        }
        super::state::clear(&select);
        super::sync::selected_attr(option, selected);
        super::sync::value_attr(&select);
        super::objects::refresh(&select);
    } else {
        super::sync::selected_attr(option, selected);
    }
}

fn multiple(select: &DomHandle) -> bool {
    matches!(select.node(), Some(Node::Element(el)) if el.attrs.contains_key("multiple"))
}
