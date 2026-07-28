use super::*;

#[path = "class_list.rs"]
mod class_list;

type FieldInit = fn(&mut HashMap<String, JsValue>, Option<&JsValue>);

#[derive(Clone, Copy)]
pub(super) struct EventClass {
    name: &'static str,
    fields: FieldInit,
    custom_init: bool,
}

impl EventClass {
    const fn new(name: &'static str, fields: FieldInit, custom_init: bool) -> Self {
        Self {
            name,
            fields,
            custom_init,
        }
    }

    pub(super) fn name(self) -> &'static str {
        self.name
    }

    pub(super) fn insert_fields(self, map: &mut HashMap<String, JsValue>, init: Option<&JsValue>) {
        (self.fields)(map, init);
    }

    pub(super) fn has_custom_init(self) -> bool {
        self.custom_init
    }
}

pub(super) fn all() -> [EventClass; class_list::COUNT] {
    class_list::all()
}
