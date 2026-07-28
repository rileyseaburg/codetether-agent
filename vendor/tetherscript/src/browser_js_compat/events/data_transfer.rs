use super::*;

#[path = "data_transfer/data.rs"]
mod data;
#[path = "data_transfer/data_clear.rs"]
mod data_clear;
#[path = "data_transfer/data_get.rs"]
mod data_get;
#[path = "data_transfer/data_set.rs"]
mod data_set;
#[path = "data_transfer/items.rs"]
mod items;
#[path = "data_transfer/items_add.rs"]
mod items_add;
#[path = "data_transfer/items_clear.rs"]
mod items_clear;
#[path = "data_transfer/items_methods.rs"]
mod items_methods;
#[path = "data_transfer/items_remove.rs"]
mod items_remove;
#[path = "data_transfer/model.rs"]
mod model;
#[path = "data_transfer/object.rs"]
mod object;

pub(super) fn install(window: &mut HashMap<String, JsValue>) {
    window.insert(
        "DataTransfer".into(),
        native("DataTransfer", None, |_| Ok(object::create())),
    );
}
