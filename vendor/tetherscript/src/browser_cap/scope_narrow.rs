//! Capability attenuation for browser authorities.

use std::rc::Rc;

use crate::capability::Authority;
use crate::value::Value;

use super::authority::BrowserAuthority;

impl BrowserAuthority {
    pub(crate) fn narrowed(&self, params: &Value) -> Result<Rc<dyn Authority>, String> {
        let map = match params {
            Value::Map(m) => m.borrow(),
            _ => return Err("browser.narrow: expected params map".into()),
        };
        let mut next = self.clone();
        if let Some(v) = map.get("origins") {
            super::scope_apply::origins(self, &mut next, v)?;
        }
        if let Some(v) = map.get("scopes") {
            super::scope_apply::scopes(self, &mut next, v)?;
        }
        if let Some(Value::Str(s)) = map.get("path_prefix") {
            let p = (**s).clone();
            if let Some(current) = &self.path_prefix {
                if !p.starts_with(current) {
                    return Err("browser.narrow: path_prefix must extend current prefix".into());
                }
            }
            next.path_prefix = Some(p);
        }
        if let Some(Value::Str(s)) = map.get("storage_scope") {
            next.storage_scope = Some((**s).clone());
        }
        if let Some(Value::Bool(b)) = map.get("human_approval") {
            next.human_approval = *b || self.human_approval;
        }
        Ok(Rc::new(next))
    }
}
