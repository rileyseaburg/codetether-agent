use super::*;

#[path = "class_list_arrays.rs"]
mod arrays;
#[path = "class_list_for_each.rs"]
mod for_each;
#[path = "class_list_iter.rs"]
mod iter;
#[path = "class_list_read.rs"]
mod read;
#[path = "class_list_sync.rs"]
mod sync;
#[path = "class_list_tokens.rs"]
mod tokens;
#[path = "class_list_validate.rs"]
mod validate;
#[path = "class_list_write.rs"]
mod write;

type ListMap = HashMap<String, JsValue>;
type ListObject = Rc<RefCell<ListMap>>;
type ListWeak = Weak<RefCell<ListMap>>;

pub(super) fn object(handle: DomHandle) -> JsValue {
    let obj = Rc::new(RefCell::new(HashMap::new()));
    sync::object(&obj, &handle);
    let weak = Rc::downgrade(&obj);
    {
        let mut map = obj.borrow_mut();
        read::install(&mut map, &handle);
        write::install(&mut map, &handle, &weak);
        iter::install(&mut map, &handle, &weak);
        sync::install_value_setter(&mut map, &handle, &weak);
    }
    JsValue::Object(obj)
}
