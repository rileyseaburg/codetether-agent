use super::*;

pub(super) fn create(args: &[JsValue]) -> Result<JsValue, String> {
    let width = number_arg(args, 0);
    let height = number_arg(args, 1);
    let mut attrs = HashMap::new();
    insert_dimension_attr(&mut attrs, "width", width);
    insert_dimension_attr(&mut attrs, "height", height);
    let image = detached_node_object(Node::Element(Element {
        tag: "img".into(),
        attrs,
        children: Vec::new(),
    }));
    decorate(&image, width.unwrap_or(0.0), height.unwrap_or(0.0));
    Ok(image)
}

fn insert_dimension_attr(attrs: &mut HashMap<String, String>, name: &str, value: Option<f64>) {
    if let Some(value) = value {
        attrs.insert(name.into(), format!("{}", value as i64));
    }
}

fn decorate(image: &JsValue, width: f64, height: f64) {
    let Some(obj) = object(image) else {
        return;
    };
    let mut obj = obj.borrow_mut();
    obj.insert("nodeName".into(), JsValue::String("IMG".into()));
    obj.insert("width".into(), JsValue::Number(width));
    obj.insert("height".into(), JsValue::Number(height));
    obj.insert("naturalWidth".into(), JsValue::Number(width));
    obj.insert("naturalHeight".into(), JsValue::Number(height));
    obj.insert("complete".into(), JsValue::Bool(true));
    obj.insert("decoding".into(), JsValue::String("auto".into()));
    obj.insert("loading".into(), JsValue::String("eager".into()));
}
