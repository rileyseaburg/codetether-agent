//! Parameter substitution and example rendering.

use super::QueryTemplate;
impl QueryTemplate {
    /// Render this template with the given parameter values.
    ///
    /// # Examples
    /// ```
    /// use codetether_rlm::{QueryTemplate};
    /// let template = QueryTemplate::find("grep_find_occurrences").unwrap();
    /// assert_eq!(template.render(&["fn", "main.rs"]), "Find all occurrences of fn in main.rs");
    /// ```
    pub fn render(&self, values: &[&str]) -> String {
        let mut result = self.template.to_string();

        for (i, value) in values.iter().enumerate() {
            if i < self.params.len() {
                let placeholder = format!("{{{}}}", self.params[i].name);
                result = result.replace(&placeholder, value);
            }
        }

        result
    }

    /// Generate a random example query (for testing).
    ///
    /// # Examples
    /// ```
    /// use codetether_rlm::{QueryTemplate};
    /// let template = QueryTemplate::find("grep_find_occurrences").unwrap();
    /// assert_eq!(template.random_example(), template.render(template.example));
    /// ```
    #[allow(dead_code)]
    pub fn random_example(&self) -> String {
        self.render(self.example)
    }
}
