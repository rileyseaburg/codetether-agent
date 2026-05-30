use serde_json::{Value, json};

pub struct EditArgs<'a> {
    pub path: &'a str,
    pub old_string: Option<&'a str>,
    pub new_string: Option<&'a str>,
    pub instruction: Option<&'a str>,
    pub update: Option<&'a str>,
    pub replace_all: bool,
}

impl<'a> EditArgs<'a> {
    pub fn parse(args: &'a Value) -> Result<Self, Value> {
        let path = args["path"].as_str().ok_or_else(
            || json!({"code":"INVALID_ARGUMENT","message":"path is required","field":"path"}),
        )?;
        Ok(Self {
            path,
            old_string: args["old_string"].as_str(),
            new_string: args["new_string"].as_str(),
            instruction: args["instruction"].as_str(),
            update: args["update"].as_str(),
            replace_all: args["replace_all"].as_bool().unwrap_or(false),
        })
    }

    pub fn has_pair(&self) -> bool {
        self.old_string.is_some() && self.new_string.is_some()
    }
}
