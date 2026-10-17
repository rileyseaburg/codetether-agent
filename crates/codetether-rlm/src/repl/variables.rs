//! Mutable named string values for expression evaluation.
use super::*;
impl RlmRepl {
    /// Set a variable
    pub fn set_var(&mut self, name: &str, value: String) {
        self.variables.insert(name.to_string(), value);
    }

    /// Get a variable
    pub fn get_var(&self, name: &str) -> Option<&str> {
        self.variables.get(name).map(|s| s.as_str())
    }
}
