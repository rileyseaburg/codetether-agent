use super::super::*;

pub(super) fn select(option: &DomHandle) -> Option<DomHandle> {
    let mut path = option.path.clone();
    while path.pop().is_some() {
        let handle = DomHandle {
            root: option.root.clone(),
            path: path.clone(),
        };
        if matches!(handle.node(), Some(Node::Element(el)) if el.tag == "select") {
            return Some(handle);
        }
    }
    None
}

pub(super) fn index(option: &DomHandle) -> isize {
    select(option)
        .and_then(|select| {
            super::handles::all(&select)
                .iter()
                .position(|handle| handle.path == option.path)
        })
        .map(|index| index as isize)
        .unwrap_or(-1)
}
