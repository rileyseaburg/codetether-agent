//! Clone implementation for browser pages.

use std::cell::RefCell;
use std::rc::Rc;

use crate::browser_agent::page::BrowserPage;

impl Clone for BrowserPage {
    fn clone(&self) -> Self {
        Self {
            session: self.session.clone(),
            viewport_width: self.viewport_width,
            viewport_height: self.viewport_height,
            device_scale: self.device_scale,
            media: self.media,
            wait_options: self.wait_options,
            resource_limits: self.resource_limits,
            security_policy: self.security_policy.clone(),
            permissions: self.permissions.clone(),
            geolocation: self.geolocation.clone(),
            download_records: self.download_records.clone(),
            resources: self.resources.clone(),
            network_routes: Rc::new(RefCell::new(self.network_routes.borrow().clone())),
            action_trace: self.action_trace.clone(),
            pointer_capture: self.pointer_capture.clone(),
            frame_windows: self.frame_windows.clone(),
            runtime: None,
            navigation: self.navigation.clone(),
            history: self.history.clone(),
            context_state: self.context_state.clone(),
            event_log: self.event_log.clone(),
        }
    }
}
