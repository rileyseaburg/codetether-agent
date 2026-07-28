use super::{BrowserPage, ColorScheme, ForcedColors, MediaEmulation, ReducedMotion};

#[test]
fn media_defaults_are_no_preference() {
    let page = BrowserPage::new(Default::default());
    let media = page.media();
    assert_eq!(media.color_scheme, ColorScheme::NoPreference);
    assert_eq!(media.reduced_motion, ReducedMotion::NoPreference);
    assert_eq!(media.forced_colors, ForcedColors::None);
}

#[test]
fn media_setters_update_snapshot() {
    let mut page = BrowserPage::new(Default::default());
    page.set_color_scheme(ColorScheme::Dark);
    page.set_reduced_motion(ReducedMotion::Reduce);
    page.set_forced_colors(ForcedColors::Active);
    assert_eq!(
        page.media(),
        MediaEmulation {
            color_scheme: ColorScheme::Dark,
            reduced_motion: ReducedMotion::Reduce,
            forced_colors: ForcedColors::Active,
        }
    );
}

#[test]
fn media_clone_debug_and_eq_are_stable() {
    let mut page = BrowserPage::from_html("mem://media", "<main>Media</main>");
    page.set_media_emulation(MediaEmulation {
        color_scheme: ColorScheme::Dark,
        reduced_motion: ReducedMotion::Reduce,
        forced_colors: ForcedColors::Active,
    });
    let clone = page.clone();
    assert_eq!(clone, page);
    assert!(format!("{page:?}").contains("media"));
    let mut changed = clone;
    changed.set_color_scheme(ColorScheme::Light);
    assert_ne!(changed, page);
}
