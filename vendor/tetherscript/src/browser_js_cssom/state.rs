use std::cell::RefCell;
use std::rc::Rc;

use crate::browser::Document;

use super::build;
use super::model::{Rule, Sheet};
use super::sync;

#[derive(Clone)]
pub(super) struct Cssom {
    pub(super) sheets: Rc<RefCell<Vec<Sheet>>>,
    css: Rc<RefCell<String>>,
    on_change: Rc<dyn Fn(&str)>,
}

impl Cssom {
    pub(super) fn new(document: &Document, css: String, on_change: Rc<dyn Fn(&str)>) -> Self {
        let this = Self {
            sheets: Rc::new(RefCell::new(build::sheets(document, &css))),
            css: Rc::new(RefCell::new(css)),
            on_change,
        };
        (this.on_change)(&this.css.borrow());
        this
    }

    pub(super) fn len(&self) -> usize {
        self.sheets.borrow().len()
    }

    pub(super) fn rules(&self, sheet: usize) -> Vec<Rule> {
        self.sheets
            .borrow()
            .get(sheet)
            .map(|sheet| sheet.rules.clone())
            .unwrap_or_default()
    }

    pub(super) fn sync(&self) {
        let css = sync::css(&self.sheets.borrow());
        *self.css.borrow_mut() = css.clone();
        (self.on_change)(&css);
    }
}
