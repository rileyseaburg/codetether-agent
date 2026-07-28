use super::*;

pub(super) fn install(obj: &mut ListMap, handle: &DomHandle, weak: &ListWeak) {
    let h = handle.clone();
    obj.insert(
        "keys".into(),
        native("classList.keys", Some(0), move |_| {
            Ok(arrays::keys(&tokens::current(&h)))
        }),
    );
    let h = handle.clone();
    obj.insert(
        "values".into(),
        native("classList.values", Some(0), move |_| {
            Ok(arrays::values(&tokens::current(&h)))
        }),
    );
    let h = handle.clone();
    obj.insert(
        "entries".into(),
        native("classList.entries", Some(0), move |_| {
            Ok(arrays::entries(&tokens::current(&h)))
        }),
    );
    let h = handle.clone();
    let list = weak.clone();
    obj.insert(
        "forEach".into(),
        native("classList.forEach", None, move |args| {
            for_each::run(&h, &list, args)
        }),
    );
}
