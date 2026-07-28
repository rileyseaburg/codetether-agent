use super::super::*;
use super::{all_callbacks, all_callbacks::Progress, link, target::Target};

pub(super) fn run(args: &[JsValue]) -> Result<JsValue, String> {
    let items = super::array_arg(args, "Promise.all")?;
    let items = items.borrow().clone();
    let (target, promise) = Target::new();
    if items.is_empty() {
        target.settle(state::PromiseState::Fulfilled(JsValue::Array(Rc::new(
            RefCell::new(Vec::new()),
        ))));
        return Ok(promise);
    }
    let progress = Progress::new(items.len());
    for (index, item) in items.into_iter().enumerate() {
        match state::settle(item.clone()) {
            state::PromiseState::Fulfilled(value) => progress.fill(index, value, &target),
            state::PromiseState::Rejected(reason) => {
                target.settle(state::PromiseState::Rejected(reason));
                break;
            }
            state::PromiseState::Pending => {
                let ok = progress.resolver(index, target.clone());
                let err = all_callbacks::rejecter(target.clone());
                link::attach_pending(&item, ok, err)?;
            }
        }
    }
    Ok(promise)
}
