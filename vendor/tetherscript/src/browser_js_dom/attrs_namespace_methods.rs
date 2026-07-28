use super::super::super::*;
use super::super::attr_update;
use super::attrs_namespace_args as args;
use super::attrs_namespace_meta::{self, AttrNs};

pub(super) fn get(handle: &DomHandle, args: &[JsValue]) -> Result<JsValue, String> {
    let (namespace, local) = args::ns_local(args);
    Ok(
        super::attrs_namespace_lookup::qualified(handle, &namespace, &local)
            .and_then(|name| attr_update::value(handle, &name))
            .map(JsValue::String)
            .unwrap_or(JsValue::Null),
    )
}

pub(super) fn set(handle: &DomHandle, args: &[JsValue]) -> Result<JsValue, String> {
    let namespace = args::namespace(args.first());
    let qualified = args.get(1).unwrap_or(&JsValue::Undefined).display();
    let value = args.get(2).unwrap_or(&JsValue::Undefined).display();
    let local = args::local_name(&qualified);
    if let Some(old) = attrs_namespace_meta::remove(handle, &namespace, &local) {
        if old != qualified {
            attr_update::remove(handle, &old)?;
        }
    }
    attr_update::set(handle, &qualified, value)?;
    attrs_namespace_meta::set(
        handle,
        AttrNs {
            qualified,
            local,
            namespace,
        },
    );
    Ok(JsValue::Undefined)
}

pub(super) fn has(handle: &DomHandle, args: &[JsValue]) -> Result<JsValue, String> {
    let (namespace, local) = args::ns_local(args);
    Ok(JsValue::Bool(
        super::attrs_namespace_lookup::qualified(handle, &namespace, &local).is_some(),
    ))
}

pub(super) fn remove(handle: &DomHandle, args: &[JsValue]) -> Result<JsValue, String> {
    let (namespace, local) = args::ns_local(args);
    if let Some(name) = super::attrs_namespace_lookup::qualified(handle, &namespace, &local) {
        attrs_namespace_meta::remove(handle, &namespace, &local);
        attr_update::remove(handle, &name)?;
    }
    Ok(JsValue::Undefined)
}
