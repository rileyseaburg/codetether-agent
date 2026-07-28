//! Idle queue draining.

use super::*;

pub(super) fn drain(
    timers: Rc<RefCell<TimerQueue>>,
    window: JsValue,
    drained: &mut usize,
) -> Result<bool, String> {
    let mut ran = false;
    loop {
        let task = { timers.borrow_mut().idle_callbacks.pop_front() };
        let Some(task) = task else {
            break;
        };
        ran = true;
        *drained += 1;
        if *drained > MAX_TIMER_DRAIN {
            return Err("requestIdleCallback: exceeded deterministic drain limit".into());
        }
        js::call_function_with_this(task.callback, window.clone(), &task.args)?;
        drain_microtasks(timers.clone(), window.clone())?;
    }
    Ok(ran)
}
