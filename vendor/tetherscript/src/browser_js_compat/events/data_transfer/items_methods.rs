use super::*;

pub(super) fn install(object: &items::ItemObject, items: &items::SharedItems) {
    items_add::install(object, items);
    items_clear::install(object, items);
    items_remove::install(object, items);
}
