//! Runtime canvas surface store.

use super::{surface::Surface, *};

thread_local! { static SURFACES: RefCell<HashMap<String, Surface>> = RefCell::new(HashMap::new()); }

pub(super) fn reset_all() {
    SURFACES.with(|surfaces| surfaces.borrow_mut().clear());
}

pub(super) fn reset_surface(handle: &DomHandle) {
    let (width, height) = super::dimensions::dimensions(handle);
    let surface = Surface::new(width, height);
    SURFACES.with(|surfaces| {
        surfaces
            .borrow_mut()
            .insert(handle.event_key(), surface.clone())
    });
    super::image::sync_attrs(handle, &surface);
}

pub(super) fn ensure(handle: &DomHandle) {
    let key = handle.event_key();
    let (width, height) = super::dimensions::dimensions(handle);
    let surface = SURFACES.with(|surfaces| {
        let mut surfaces = surfaces.borrow_mut();
        let refresh = surfaces
            .get(&key)
            .is_none_or(|s| s.width != width || s.height != height);
        if refresh {
            surfaces.insert(key.clone(), Surface::new(width, height));
        }
        surfaces
            .get(&key)
            .cloned()
            .unwrap_or_else(|| Surface::new(width, height))
    });
    super::image::sync_attrs(handle, &surface);
}

pub(super) fn mutate(handle: &DomHandle, f: impl FnOnce(&mut Surface)) {
    ensure(handle);
    let key = handle.event_key();
    let surface = SURFACES.with(|surfaces| {
        let mut surfaces = surfaces.borrow_mut();
        let surface = surfaces.get_mut(&key).expect("canvas surface exists");
        f(surface);
        surface.clone()
    });
    super::image::sync_attrs(handle, &surface);
}

pub(super) fn with_surface<T>(handle: &DomHandle, f: impl FnOnce(&Surface) -> T) -> T {
    ensure(handle);
    let key = handle.event_key();
    SURFACES.with(|surfaces| f(surfaces.borrow().get(&key).expect("canvas surface exists")))
}
