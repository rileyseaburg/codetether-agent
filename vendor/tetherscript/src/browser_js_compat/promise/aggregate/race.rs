use super::super::*;
use super::{link, target::Target};

pub(super) fn run(args: &[JsValue]) -> Result<JsValue, String> {
    let items = super::array_arg(args, "Promise.race")?;
    let (target, promise) = Target::new();
    for item in items.borrow().iter() {
        match state::settle(item.clone()) {
            state::PromiseState::Fulfilled(value) => {
                target.settle(state::PromiseState::Fulfilled(value));
                break;
            }
            state::PromiseState::Rejected(reason) => {
                target.settle(state::PromiseState::Rejected(reason));
                break;
            }
            state::PromiseState::Pending => {
                let ok = settler(target.clone(), true);
                let err = settler(target.clone(), false);
                link::attach_pending(item, ok, err)?;
            }
        }
    }
    Ok(promise)
}

fn settler(target: Target, fulfill: bool) -> JsValue {
    native("Promise.race.settle", Some(1), move |args| {
        let value = args.first().cloned().unwrap_or(JsValue::Undefined);
        let next = if fulfill {
            state::PromiseState::Fulfilled(value)
        } else {
            state::PromiseState::Rejected(value)
        };
        target.settle(next);
        Ok(JsValue::Undefined)
    })
}
