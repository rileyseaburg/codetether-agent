use std::cell::RefCell;
use std::rc::Rc;

type DrainHook = Rc<dyn Fn() -> Result<bool, String>>;

thread_local! {
    static DRAIN: RefCell<Vec<DrainHook>> = RefCell::new(Vec::new());
}

struct DrainScope;

impl Drop for DrainScope {
    fn drop(&mut self) {
        DRAIN.with(|drain| {
            drain.borrow_mut().pop();
        });
    }
}

pub(crate) fn with_drain<T>(
    hook: Rc<dyn Fn() -> Result<bool, String>>,
    work: impl FnOnce() -> T,
) -> T {
    DRAIN.with(|drain| drain.borrow_mut().push(hook));
    let _scope = DrainScope;
    work()
}

pub(super) fn drain_once() -> Result<bool, String> {
    let hook = DRAIN.with(|drain| drain.borrow().last().cloned());
    hook.map_or(Ok(false), |hook| hook())
}
