use super::{context::Context, dispatch};
use std::panic::{AssertUnwindSafe, catch_unwind};
use windows_sys::Win32::{Foundation::*, UI::WindowsAndMessaging::PostQuitMessage};

pub(super) fn handle(
    context: &Context,
    hwnd: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> Option<LRESULT> {
    let outcome = catch_unwind(AssertUnwindSafe(|| {
        dispatch::handle(context, hwnd, message, wparam, lparam)
    }));
    match outcome {
        Ok(Ok(result)) => result,
        other => {
            context
                .interrupt
                .store(true, std::sync::atomic::Ordering::Release);
            let error = match other {
                Ok(Err(error)) => error,
                Err(_) => anyhow::anyhow!("Native window callback panicked"),
                Ok(Ok(_)) => unreachable!(),
            };
            if let Ok(mut failure) = context.failure.try_borrow_mut() {
                if failure.is_none() {
                    *failure = Some(error);
                }
            }
            // SAFETY: stop this UI thread; Window's guard performs destruction.
            unsafe {
                PostQuitMessage(1);
            }
            Some(0)
        }
    }
}
