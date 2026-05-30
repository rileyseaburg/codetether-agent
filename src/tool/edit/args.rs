use serde_json::{Value, json};

use super::super::ToolResult;

pub struct EditArgs<'a> {
    pub path: &'a str,
    pub old_string: Option<&'a str>,
    pub new_string: Option<&'a str>,
    pub instruction: Option<&'a str>,
    pub update: Option<&'a str>,
    pub replace_all: bool,
}

impl<'a> EditArgs<'a> {
    pub fn parse(args: &'a Value) -> Result<Self, ToolResult> {
        let Some(path) = args["path"].as_str() else {
            return Err(ToolResult::structured_error(
                "INVALID_ARGUMENT",
                "edit",
                "path is required",
                Some(vec!["path"]),
                Some(json!({"path":"src/main.rs","old_string":"old","new_string":"new"})),
            ));
        };
        Ok(Self {
            path,
            old_string: args["old_string"].as_str(),
            new_string: args["new_string"].as_str(),
            instruction: args["instruction"].as_str(),
            update: args["update"].as_str(),
            replace_all: args["replace_all"].as_bool().unwrap_or(false),
        })
    }
}

pub fn required<'a>(
    value: Option<&'a str>,
    field: &str,
    path: &str,
) -> Result<&'a str, ToolResult> {
    value.ok_or_else(|| {
        ToolResult::structured_error(
            "INVALID_ARGUMENT",
            "edit",
            &format!("{field} is required unless Morph backend is enabled and instruction/update are provided"),
            Some(vec![field]),
            Some(json!({"path": path, "old_string": "old text", "new_string": "new text"})),
        )
    })
}
