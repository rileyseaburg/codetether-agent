use super::super::*;
use super::calls;

pub(super) fn changed(
    handle: &DomHandle,
    name: &str,
    old_value: Option<String>,
    new_value: Option<String>,
) -> Result<(), String> {
    if old_value == new_value {
        return Ok(());
    }
    let Some(definition) = definition_for(handle, name) else {
        return Ok(());
    };
    let args = [
        JsValue::String(name.into()),
        util::js_option(old_value),
        util::js_option(new_value),
    ];
    calls::call(
        &definition.value,
        "attributeChangedCallback",
        node_object(handle.clone()),
        &args,
    )
}

fn definition_for(handle: &DomHandle, name: &str) -> Option<registry::CustomDefinition> {
    let tag = handle.node().and_then(|node| util::tag_name(&node))?;
    let definition = registry::get(&tag)?;
    definition
        .observed
        .iter()
        .any(|item| item == name)
        .then_some(definition)
}
