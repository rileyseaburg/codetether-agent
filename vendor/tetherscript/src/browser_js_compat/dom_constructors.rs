use super::*;

#[path = "dom_constructors/args.rs"]
mod args;
#[path = "dom_constructors/image.rs"]
mod image;
#[path = "dom_constructors/json.rs"]
mod json;
#[path = "dom_constructors/matrix.rs"]
mod matrix;
#[path = "dom_constructors/node.rs"]
mod node;
#[path = "dom_constructors/number.rs"]
mod number;
#[path = "dom_constructors/option.rs"]
mod option;
#[path = "dom_constructors/point.rs"]
mod point;
#[path = "dom_constructors/rect.rs"]
mod rect;
#[path = "dom_constructors/unsupported.rs"]
mod unsupported;

use args::{bool_arg, number_arg, object, string_arg};

#[cfg(test)]
#[path = "tests_dom_constructor_globals.rs"]
mod tests_dom_constructor_globals;
#[cfg(test)]
#[path = "tests_dom_constructors.rs"]
mod tests_dom_constructors;
#[cfg(test)]
#[path = "tests_geometry_constructors.rs"]
mod tests_geometry_constructors;

pub(super) fn install(window: &mut HashMap<String, JsValue>) {
    window.insert("Node".into(), node::create());
    window.insert("Image".into(), native("Image", None, image::create));
    window.insert("Option".into(), native("Option", None, option::create));
    window.insert("DOMPoint".into(), point::constructor());
    window.insert("DOMRect".into(), rect::constructor());
    window.insert("DOMMatrix".into(), matrix::constructor("DOMMatrix"));
    window.insert(
        "DOMMatrixReadOnly".into(),
        matrix::constructor("DOMMatrixReadOnly"),
    );
    unsupported::install(window);
}
