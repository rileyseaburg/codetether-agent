use super::*;

#[path = "url_pattern/constructor.rs"]
mod constructor;
#[path = "url_pattern/exec.rs"]
mod exec;
#[path = "url_pattern/glob.rs"]
mod glob;
#[path = "url_pattern/input.rs"]
mod input;
#[path = "url_pattern/matcher.rs"]
mod matcher;
#[path = "url_pattern/model.rs"]
mod model;
#[path = "url_pattern/norm.rs"]
mod norm;
#[path = "url_pattern/object.rs"]
mod object;
#[path = "url_pattern/object_input.rs"]
mod object_input;
#[path = "url_pattern/path.rs"]
mod path;
#[path = "url_pattern/string_input.rs"]
mod string_input;

#[cfg(test)]
#[path = "tests_url_pattern.rs"]
mod tests_url_pattern;

pub(super) fn install(window: &mut HashMap<String, JsValue>) {
    window.insert("URLPattern".into(), constructor::value());
}
