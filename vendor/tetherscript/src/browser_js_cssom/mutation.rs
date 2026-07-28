use crate::js::JsValue;

use super::index;
use super::parse_rule;
use super::state::Cssom;

impl Cssom {
    pub(super) fn insert_rule(
        &self,
        sheet: usize,
        source: &str,
        index: Option<&JsValue>,
    ) -> Result<usize, String> {
        let rule = parse_rule::parse(source)?;
        let mut sheets = self.sheets.borrow_mut();
        let rules = &mut sheets
            .get_mut(sheet)
            .ok_or_else(|| format!("insertRule: stylesheet {sheet} does not exist"))?
            .rules;
        let index = index::insert(index, rules.len())?;
        rules.insert(index, rule);
        drop(sheets);
        self.sync();
        Ok(index)
    }

    pub(super) fn delete_rule(&self, sheet: usize, index: &JsValue) -> Result<(), String> {
        let mut sheets = self.sheets.borrow_mut();
        let rules = &mut sheets
            .get_mut(sheet)
            .ok_or_else(|| format!("deleteRule: stylesheet {sheet} does not exist"))?
            .rules;
        let index = index::delete(index, rules.len())?;
        rules.remove(index);
        drop(sheets);
        self.sync();
        Ok(())
    }
}
