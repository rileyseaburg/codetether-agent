use super::{
    control_spec::specs,
    ids,
    text::{failure, wide},
};
use anyhow::Result;
use windows_sys::Win32::{Foundation::HWND, UI::WindowsAndMessaging::*};

pub(super) struct Controls {
    pub(super) picker: HWND,
    pub(super) status: HWND,
    pub(super) code: HWND,
    pub(super) pair: HWND,
    pub(super) children: Vec<(HWND, [i32; 4])>,
}

impl Controls {
    pub(super) fn create(parent: HWND) -> Result<Self> {
        let null = std::ptr::null_mut();
        let mut controls = Self {
            picker: null,
            status: null,
            code: null,
            pair: null,
            children: Vec::new(),
        };
        for spec in specs() {
            let title = wide(spec.text);
            let disabled = if spec.enabled { 0 } else { WS_DISABLED };
            // SAFETY: parent is live; Windows copies the title; children belong to parent.
            let child = unsafe {
                CreateWindowExW(
                    0,
                    spec.class,
                    title.as_ptr(),
                    WS_CHILD | WS_VISIBLE | disabled | spec.style,
                    0,
                    0,
                    0,
                    0,
                    parent,
                    spec.id as usize as HMENU,
                    std::ptr::null_mut(),
                    std::ptr::null(),
                )
            };
            if child.is_null() {
                return Err(failure("Cannot create local controls"));
            }
            if spec.id == ids::PICKER {
                controls.picker = child;
            }
            if spec.id == ids::STATUS {
                controls.status = child;
            }
            if spec.id == ids::CODE {
                controls.code = child;
            }
            if spec.id == ids::PAIR {
                controls.pair = child;
            }
            controls.children.push((child, spec.bounds));
        }
        Ok(controls)
    }

    pub(super) fn set_status(&self, text: &str) {
        let text = wide(text);
        // SAFETY: this child belongs to the live main window and text is terminated.
        unsafe {
            SetWindowTextW(self.status, text.as_ptr());
        }
    }
}
