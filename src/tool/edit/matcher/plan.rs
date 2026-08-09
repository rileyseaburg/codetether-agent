pub struct MatchPlan {
    pub target: String,
    pub count: usize,
    pub replace_all: bool,
    pub strategy: &'static str,
}

impl MatchPlan {
    pub fn apply(&self, content: &str, new_string: &str) -> String {
        match self.replace_all {
            true => content.replace(&self.target, new_string),
            false => content.replacen(&self.target, new_string, 1),
        }
    }

    pub fn confirm_old_string<'a>(&'a self, requested: &'a str) -> &'a str {
        if self.strategy == "exact" {
            requested
        } else {
            &self.target
        }
    }

    pub fn strategy(&self) -> &'static str {
        self.strategy
    }

    pub fn count(&self) -> usize {
        if self.replace_all { self.count } else { 1 }
    }
}
