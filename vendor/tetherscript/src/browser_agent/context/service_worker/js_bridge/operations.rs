//! Drain parsing for service-worker bridge operations.

use crate::js::JsValue;

pub(super) enum BridgeOperation {
    Register {
        scope: String,
        script_url: String,
    },
    CacheStorageDelete {
        cache_name: String,
    },
    CacheDelete {
        cache_name: String,
        request_url: String,
    },
}

pub(super) fn parse(value: &JsValue) -> Vec<BridgeOperation> {
    let JsValue::Array(items) = value else {
        return Vec::new();
    };
    items.borrow().iter().filter_map(parse_one).collect()
}

fn parse_one(value: &JsValue) -> Option<BridgeOperation> {
    let JsValue::Array(items) = value else {
        return None;
    };
    let items = items.borrow();
    match text(items.first()?)?.as_str() {
        "register" => Some(BridgeOperation::Register {
            scope: text(items.get(1)?)?,
            script_url: text(items.get(2)?)?,
        }),
        "cacheStorageDelete" => Some(BridgeOperation::CacheStorageDelete {
            cache_name: text(items.get(1)?)?,
        }),
        "cacheDelete" => Some(BridgeOperation::CacheDelete {
            cache_name: text(items.get(1)?)?,
            request_url: text(items.get(2)?)?,
        }),
        _ => None,
    }
}

fn text(value: &JsValue) -> Option<String> {
    Some(value.display())
}
