//! JSON parsing and encoding.
//!
//! Provides `json_parse`, `json_encode`, and `json_encode_pretty` built-ins.
//! This module is dependency-free and implements the JSON subset needed by
//! TetherScript and the LSP server directly.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::value::Value;

/// Parse a JSON string into a TetherScript value.
///
/// ```ignore
/// let data = json_parse("{\"name\":\"TetherScript\",\"version\":1}");
/// println(data.name);   // TetherScript
/// println(data.version); // 1
/// ```
pub fn parse(arg: &Value) -> Result<Value, String> {
    let s = match arg {
        Value::Str(s) => s.as_str(),
        other => {
            return Err(format!(
                "json_parse: expected string, got {}",
                other.type_name()
            ))
        }
    };
    parse_str(s).map_err(|error| format!("json_parse: {}", error))
}

/// Encode a TetherScript value into a JSON string.
///
/// ```ignore
/// let data = map();
/// data.name = "TetherScript";
/// data.stable = false;
/// let s = json_encode(data);
/// // s == "{\"name\":\"TetherScript\",\"stable\":false}"
/// ```
pub fn encode(arg: &Value) -> Result<Value, String> {
    Ok(Value::Str(Rc::new(encode_to_string(arg)?)))
}

/// Encode a TetherScript value into a pretty-printed JSON string.
pub fn encode_pretty(arg: &Value) -> Result<Value, String> {
    let mut out = String::new();
    write_json(arg, &mut out, Some(0))?;
    Ok(Value::Str(Rc::new(out)))
}

pub(crate) fn parse_str(input: &str) -> Result<Value, String> {
    let mut parser = JsonParser {
        bytes: input.as_bytes(),
        pos: 0,
    };
    let value = parser.parse_value()?;
    parser.skip_ws();
    if !parser.done() {
        return Err(parser.error("trailing characters"));
    }
    Ok(value)
}

pub(crate) fn encode_to_string(value: &Value) -> Result<String, String> {
    let mut out = String::new();
    write_json(value, &mut out, None)?;
    Ok(out)
}

struct JsonParser<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl JsonParser<'_> {
    fn parse_value(&mut self) -> Result<Value, String> {
        self.skip_ws();
        match self.peek() {
            Some(b'n') => {
                self.expect_literal("null")?;
                Ok(Value::Nil)
            }
            Some(b't') => {
                self.expect_literal("true")?;
                Ok(Value::Bool(true))
            }
            Some(b'f') => {
                self.expect_literal("false")?;
                Ok(Value::Bool(false))
            }
            Some(b'"') => Ok(Value::Str(Rc::new(self.parse_string()?))),
            Some(b'[') => self.parse_array(),
            Some(b'{') => self.parse_object(),
            Some(b'-' | b'0'..=b'9') => self.parse_number(),
            Some(byte) => Err(self.error(&format!("unexpected byte 0x{byte:02x}"))),
            None => Err(self.error("unexpected end of input")),
        }
    }

    fn parse_array(&mut self) -> Result<Value, String> {
        self.expect_byte(b'[')?;
        self.skip_ws();
        let mut items = Vec::new();
        if self.eat(b']') {
            return Ok(Value::List(Rc::new(RefCell::new(items))));
        }
        loop {
            items.push(self.parse_value()?);
            self.skip_ws();
            if self.eat(b']') {
                break;
            }
            self.expect_byte(b',')?;
        }
        Ok(Value::List(Rc::new(RefCell::new(items))))
    }

    fn parse_object(&mut self) -> Result<Value, String> {
        self.expect_byte(b'{')?;
        self.skip_ws();
        let mut map = HashMap::new();
        if self.eat(b'}') {
            return Ok(Value::Map(Rc::new(RefCell::new(map))));
        }
        loop {
            self.skip_ws();
            let key = self.parse_string()?;
            self.skip_ws();
            self.expect_byte(b':')?;
            let value = self.parse_value()?;
            map.insert(key, value);
            self.skip_ws();
            if self.eat(b'}') {
                break;
            }
            self.expect_byte(b',')?;
        }
        Ok(Value::Map(Rc::new(RefCell::new(map))))
    }

    fn parse_string(&mut self) -> Result<String, String> {
        self.expect_byte(b'"')?;
        let mut out = String::new();
        while let Some(byte) = self.next() {
            match byte {
                b'"' => return Ok(out),
                b'\\' => self.parse_escape(&mut out)?,
                0x00..=0x1f => return Err(self.error("control character in string")),
                _ => {
                    let start = self.pos - 1;
                    while let Some(next) = self.peek() {
                        if next == b'"' || next == b'\\' || next <= 0x1f {
                            break;
                        }
                        self.pos += 1;
                    }
                    let s = std::str::from_utf8(&self.bytes[start..self.pos])
                        .map_err(|_| self.error("invalid UTF-8 in string"))?;
                    out.push_str(s);
                }
            }
        }
        Err(self.error("unterminated string"))
    }

    fn parse_escape(&mut self, out: &mut String) -> Result<(), String> {
        match self.next() {
            Some(b'"') => out.push('"'),
            Some(b'\\') => out.push('\\'),
            Some(b'/') => out.push('/'),
            Some(b'b') => out.push('\u{0008}'),
            Some(b'f') => out.push('\u{000c}'),
            Some(b'n') => out.push('\n'),
            Some(b'r') => out.push('\r'),
            Some(b't') => out.push('\t'),
            Some(b'u') => {
                let first = self.parse_u16_escape()?;
                let scalar = if (0xd800..=0xdbff).contains(&first) {
                    self.expect_byte(b'\\')?;
                    self.expect_byte(b'u')?;
                    let second = self.parse_u16_escape()?;
                    if !(0xdc00..=0xdfff).contains(&second) {
                        return Err(self.error("invalid Unicode surrogate pair"));
                    }
                    0x10000 + (((first as u32 - 0xd800) << 10) | (second as u32 - 0xdc00))
                } else if (0xdc00..=0xdfff).contains(&first) {
                    return Err(self.error("unexpected low Unicode surrogate"));
                } else {
                    first as u32
                };
                let ch = char::from_u32(scalar)
                    .ok_or_else(|| self.error("invalid Unicode scalar value"))?;
                out.push(ch);
            }
            Some(byte) => return Err(self.error(&format!("invalid escape byte 0x{byte:02x}"))),
            None => return Err(self.error("unterminated escape")),
        }
        Ok(())
    }

    fn parse_u16_escape(&mut self) -> Result<u16, String> {
        let mut value = 0u16;
        for _ in 0..4 {
            let byte = self
                .next()
                .ok_or_else(|| self.error("unterminated Unicode escape"))?;
            value = (value << 4)
                | match byte {
                    b'0'..=b'9' => (byte - b'0') as u16,
                    b'a'..=b'f' => (byte - b'a' + 10) as u16,
                    b'A'..=b'F' => (byte - b'A' + 10) as u16,
                    _ => return Err(self.error("invalid Unicode escape")),
                };
        }
        Ok(value)
    }

    fn parse_number(&mut self) -> Result<Value, String> {
        let start = self.pos;
        self.eat(b'-');
        match self.peek() {
            Some(b'0') => self.pos += 1,
            Some(b'1'..=b'9') => {
                self.pos += 1;
                while matches!(self.peek(), Some(b'0'..=b'9')) {
                    self.pos += 1;
                }
            }
            _ => return Err(self.error("invalid number")),
        }

        let mut float = false;
        if self.eat(b'.') {
            float = true;
            let mut digits = 0;
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                digits += 1;
                self.pos += 1;
            }
            if digits == 0 {
                return Err(self.error("invalid fractional number"));
            }
        }

        if self.eat(b'e') || self.eat(b'E') {
            float = true;
            let _ = self.eat(b'+') || self.eat(b'-');
            let mut digits = 0;
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                digits += 1;
                self.pos += 1;
            }
            if digits == 0 {
                return Err(self.error("invalid exponent"));
            }
        }

        let text = std::str::from_utf8(&self.bytes[start..self.pos])
            .map_err(|_| self.error("invalid UTF-8 in number"))?;
        if !float {
            if let Ok(value) = text.parse::<i64>() {
                return Ok(Value::Int(value));
            }
        }
        let value = text
            .parse::<f64>()
            .map_err(|error| self.error(&format!("invalid number: {error}")))?;
        if !value.is_finite() {
            return Err(self.error("non-finite number"));
        }
        Ok(Value::Float(value))
    }

    fn expect_literal(&mut self, literal: &str) -> Result<(), String> {
        if self.bytes[self.pos..].starts_with(literal.as_bytes()) {
            self.pos += literal.len();
            Ok(())
        } else {
            Err(self.error(&format!("expected {literal}")))
        }
    }

    fn expect_byte(&mut self, expected: u8) -> Result<(), String> {
        match self.next() {
            Some(byte) if byte == expected => Ok(()),
            Some(byte) => {
                Err(self.error(&format!("expected byte 0x{expected:02x}, got 0x{byte:02x}")))
            }
            None => Err(self.error(&format!("expected byte 0x{expected:02x}"))),
        }
    }

    fn eat(&mut self, byte: u8) -> bool {
        if self.peek() == Some(byte) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn skip_ws(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\n' | b'\r' | b'\t')) {
            self.pos += 1;
        }
    }

    fn next(&mut self) -> Option<u8> {
        let byte = self.peek()?;
        self.pos += 1;
        Some(byte)
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    fn done(&self) -> bool {
        self.pos >= self.bytes.len()
    }

    fn error(&self, message: &str) -> String {
        format!("{message} at byte {}", self.pos)
    }
}

fn write_json(value: &Value, out: &mut String, pretty: Option<usize>) -> Result<(), String> {
    match value {
        Value::Nil => out.push_str("null"),
        Value::Bool(value) => out.push_str(if *value { "true" } else { "false" }),
        Value::Int(value) => out.push_str(&value.to_string()),
        Value::Float(value) => {
            if !value.is_finite() {
                return Err(format!(
                    "json_encode: float {} is not a valid JSON number (NaN/Inf)",
                    value
                ));
            }
            out.push_str(&value.to_string());
        }
        Value::Str(value) => write_json_string(value, out),
        Value::Bytes(value) => write_json_bytes(&value.borrow(), out, pretty)?,
        Value::List(values) => write_json_list(&values.borrow(), out, pretty)?,
        Value::Map(values) => write_json_map(&values.borrow(), out, pretty)?,
        Value::Fn(_) | Value::VmFn(_) | Value::Native(_) => {
            return Err("json_encode: cannot encode function value as JSON".to_string())
        }
        Value::Result(_) => {
            return Err(
                "json_encode: cannot encode Result value as JSON (unwrap it first)".to_string(),
            )
        }
        Value::Capability(_) => {
            return Err("json_encode: cannot encode a capability value as JSON".to_string())
        }
    }
    Ok(())
}

fn write_json_bytes(bytes: &[u8], out: &mut String, pretty: Option<usize>) -> Result<(), String> {
    out.push('[');
    if bytes.is_empty() {
        out.push(']');
        return Ok(());
    }
    let child_pretty = pretty.map(|indent| indent + 2);
    for (index, byte) in bytes.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        if let Some(indent) = child_pretty {
            out.push('\n');
            push_indent(out, indent);
        }
        out.push_str(&byte.to_string());
    }
    if let Some(indent) = pretty {
        out.push('\n');
        push_indent(out, indent);
    }
    out.push(']');
    Ok(())
}

fn write_json_list(
    values: &[Value],
    out: &mut String,
    pretty: Option<usize>,
) -> Result<(), String> {
    out.push('[');
    if values.is_empty() {
        out.push(']');
        return Ok(());
    }
    let child_pretty = pretty.map(|indent| indent + 2);
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        if let Some(indent) = child_pretty {
            out.push('\n');
            push_indent(out, indent);
        }
        write_json(value, out, child_pretty)?;
    }
    if let Some(indent) = pretty {
        out.push('\n');
        push_indent(out, indent);
    }
    out.push(']');
    Ok(())
}

fn write_json_map(
    values: &HashMap<String, Value>,
    out: &mut String,
    pretty: Option<usize>,
) -> Result<(), String> {
    out.push('{');
    if values.is_empty() {
        out.push('}');
        return Ok(());
    }
    let child_pretty = pretty.map(|indent| indent + 2);
    let mut entries: Vec<_> = values.iter().collect();
    entries.sort_by(|(left, _), (right, _)| left.cmp(right));
    for (index, (key, value)) in entries.into_iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        if let Some(indent) = child_pretty {
            out.push('\n');
            push_indent(out, indent);
        }
        write_json_string(key, out);
        out.push(':');
        if pretty.is_some() {
            out.push(' ');
        }
        write_json(value, out, child_pretty)?;
    }
    if let Some(indent) = pretty {
        out.push('\n');
        push_indent(out, indent);
    }
    out.push('}');
    Ok(())
}

fn write_json_string(value: &str, out: &mut String) {
    out.push('"');
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\u{0008}' => out.push_str("\\b"),
            '\u{000c}' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            ch if ch < ' ' => out.push_str(&format!("\\u{:04x}", ch as u32)),
            ch => out.push(ch),
        }
    }
    out.push('"');
}

fn push_indent(out: &mut String, indent: usize) {
    for _ in 0..indent {
        out.push(' ');
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_null() {
        let v = parse(&Value::Str(Rc::new("null".into()))).unwrap();
        assert!(matches!(v, Value::Nil));
    }

    #[test]
    fn parse_bool() {
        let v = parse(&Value::Str(Rc::new("true".into()))).unwrap();
        assert_eq!(v, Value::Bool(true));
        let v = parse(&Value::Str(Rc::new("false".into()))).unwrap();
        assert_eq!(v, Value::Bool(false));
    }

    #[test]
    fn parse_int() {
        let v = parse(&Value::Str(Rc::new("42".into()))).unwrap();
        assert_eq!(v, Value::Int(42));
    }

    #[test]
    fn parse_negative_int() {
        let v = parse(&Value::Str(Rc::new("-7".into()))).unwrap();
        assert_eq!(v, Value::Int(-7));
    }

    #[test]
    fn parse_float() {
        let v = parse(&Value::Str(Rc::new("3.14".into()))).unwrap();
        match v {
            Value::Float(f) => assert!((f - 3.14).abs() < 1e-10),
            other => panic!("expected Float, got {:?}", other),
        }
    }

    #[test]
    fn parse_string() {
        let v = parse(&Value::Str(Rc::new(r#""hello""#.into()))).unwrap();
        assert_eq!(v, Value::Str(Rc::new("hello".into())));
    }

    #[test]
    fn parse_string_with_escapes() {
        let v = parse(&Value::Str(Rc::new(r#""hello\nworld""#.into()))).unwrap();
        assert_eq!(v, Value::Str(Rc::new("hello\nworld".into())));
    }

    #[test]
    fn parse_unicode_escape() {
        let v = parse(&Value::Str(Rc::new(r#""hi \u263a \ud83d\ude80""#.into()))).unwrap();
        assert_eq!(v, Value::Str(Rc::new("hi \u{263a} \u{1f680}".into())));
    }

    #[test]
    fn parse_empty_array() {
        let v = parse(&Value::Str(Rc::new("[]".into()))).unwrap();
        match v {
            Value::List(xs) => assert!(xs.borrow().is_empty()),
            other => panic!("expected List, got {:?}", other),
        }
    }

    #[test]
    fn parse_array() {
        let v = parse(&Value::Str(Rc::new("[1, 2, 3]".into()))).unwrap();
        match v {
            Value::List(xs) => {
                let xs = xs.borrow();
                assert_eq!(xs.len(), 3);
                assert_eq!(xs[0], Value::Int(1));
                assert_eq!(xs[1], Value::Int(2));
                assert_eq!(xs[2], Value::Int(3));
            }
            other => panic!("expected List, got {:?}", other),
        }
    }

    #[test]
    fn parse_empty_object() {
        let v = parse(&Value::Str(Rc::new("{}".into()))).unwrap();
        match v {
            Value::Map(m) => assert!(m.borrow().is_empty()),
            other => panic!("expected Map, got {:?}", other),
        }
    }

    #[test]
    fn parse_object() {
        let v = parse(&Value::Str(Rc::new(
            r#"{"name":"TetherScript","v":1}"#.into(),
        )))
        .unwrap();
        match v {
            Value::Map(m) => {
                let m = m.borrow();
                assert_eq!(
                    m.get("name"),
                    Some(&Value::Str(Rc::new("TetherScript".into())))
                );
                assert_eq!(m.get("v"), Some(&Value::Int(1)));
            }
            other => panic!("expected Map, got {:?}", other),
        }
    }

    #[test]
    fn parse_nested() {
        let v = parse(&Value::Str(Rc::new(
            r#"{"users":[{"name":"Alice"},{"name":"Bob"}]}"#.into(),
        )))
        .unwrap();
        match v {
            Value::Map(m) => {
                let users = m.borrow().get("users").cloned().unwrap();
                match users {
                    Value::List(xs) => assert_eq!(xs.borrow().len(), 2),
                    other => panic!("expected List for users, got {:?}", other),
                }
            }
            other => panic!("expected Map, got {:?}", other),
        }
    }

    #[test]
    fn parse_rejects_non_string() {
        let err = parse(&Value::Int(42));
        assert!(err.is_err());
        assert!(err.unwrap_err().contains("expected string"));
    }

    #[test]
    fn parse_rejects_invalid_json() {
        let err = parse(&Value::Str(Rc::new("{bad".into())));
        assert!(err.is_err());
    }

    #[test]
    fn encode_null() {
        let v = encode(&Value::Nil).unwrap();
        assert_eq!(v, Value::Str(Rc::new("null".into())));
    }

    #[test]
    fn encode_bool() {
        assert_eq!(
            encode(&Value::Bool(true)).unwrap(),
            Value::Str(Rc::new("true".into()))
        );
        assert_eq!(
            encode(&Value::Bool(false)).unwrap(),
            Value::Str(Rc::new("false".into()))
        );
    }

    #[test]
    fn encode_int() {
        assert_eq!(
            encode(&Value::Int(42)).unwrap(),
            Value::Str(Rc::new("42".into()))
        );
    }

    #[test]
    fn encode_float() {
        let v = encode(&Value::Float(3.14)).unwrap();
        match v {
            Value::Str(s) => assert!(s.contains("3.14")),
            other => panic!("expected Str, got {:?}", other),
        }
    }

    #[test]
    fn encode_string() {
        let v = encode(&Value::Str(Rc::new("hello".into()))).unwrap();
        assert_eq!(v, Value::Str(Rc::new(r#""hello""#.into())));
    }

    #[test]
    fn encode_string_escapes() {
        let v = encode(&Value::Str(Rc::new("hello\nworld".into()))).unwrap();
        assert_eq!(v, Value::Str(Rc::new(r#""hello\nworld""#.into())));
    }

    #[test]
    fn encode_bytes() {
        let bytes = Value::Bytes(Rc::new(RefCell::new(vec![0, 10, 255])));
        let v = encode(&bytes).unwrap();
        assert_eq!(v, Value::Str(Rc::new("[0,10,255]".into())));
    }

    #[test]
    fn encode_list() {
        let v = encode(&Value::List(Rc::new(RefCell::new(vec![
            Value::Int(1),
            Value::Int(2),
            Value::Int(3),
        ]))))
        .unwrap();
        assert_eq!(v, Value::Str(Rc::new("[1,2,3]".into())));
    }

    #[test]
    fn encode_map() {
        let mut m = HashMap::new();
        m.insert("name".into(), Value::Str(Rc::new("TetherScript".into())));
        m.insert("v".into(), Value::Int(1));
        let v = encode(&Value::Map(Rc::new(RefCell::new(m)))).unwrap();
        match v {
            Value::Str(s) => {
                assert!(s.contains(r#""name":"TetherScript""#));
                assert!(s.contains(r#""v":1"#));
            }
            other => panic!("expected Str, got {:?}", other),
        }
    }

    #[test]
    fn encode_pretty_formats() {
        let v = encode_pretty(&Value::List(Rc::new(RefCell::new(vec![
            Value::Int(1),
            Value::Int(2),
        ]))))
        .unwrap();
        match v {
            Value::Str(s) => {
                assert!(s.contains('\n'));
                assert!(s.contains("  "));
            }
            other => panic!("expected Str, got {:?}", other),
        }
    }

    #[test]
    fn roundtrip() {
        let original = r#"{"bool":true,"int":42,"list":[1,2,3],"nil":null,"str":"hello"}"#;
        let parsed = parse(&Value::Str(Rc::new(original.into()))).unwrap();
        let encoded = encode(&parsed).unwrap();
        let reparsed = parse(&encoded).unwrap();
        let re_encoded = encode(&reparsed).unwrap();
        assert_eq!(encoded, re_encoded);
    }

    #[test]
    fn encode_rejects_function() {
        use crate::value::FnObj;
        let err = encode(&Value::Fn(Rc::new(FnObj {
            params: vec![],
            body: Rc::new(crate::ast::Block { stmts: vec![] }),
            closure: crate::value::Env::new_global(),
            name: Some("test".into()),
        })));
        assert!(err.is_err());
        assert!(err.unwrap_err().contains("function"));
    }
}
