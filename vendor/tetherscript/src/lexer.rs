//! Lexer — source text → tokens.
//!
//! Hand-written, single-pass, character-by-character. We track line/col for
//! error messages and emit explicit `Newline` tokens (even though rk uses
//! braces, newlines still matter for nice error recovery and REPL UX).

use crate::token::{Spanned, Token};

pub struct Lexer<'a> {
    src: &'a [u8],
    pos: usize,
    line: usize,
    col: usize,
}

#[derive(Debug)]
pub struct LexError {
    pub msg: String,
    pub line: usize,
    pub col: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src: src.as_bytes(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    pub fn tokenize(mut self) -> Result<Vec<Spanned>, LexError> {
        let mut out = Vec::new();
        while let Some(tok) = self.next_token()? {
            out.push(tok);
        }
        out.push(Spanned {
            token: Token::Eof,
            line: self.line,
            col: self.col,
        });
        Ok(out)
    }

    fn peek(&self) -> Option<u8> {
        self.src.get(self.pos).copied()
    }

    fn peek2(&self) -> Option<u8> {
        self.src.get(self.pos + 1).copied()
    }

    fn bump(&mut self) -> Option<u8> {
        let c = self.peek()?;
        self.pos += 1;
        if c == b'\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(c)
    }

    fn make(&self, token: Token, line: usize, col: usize) -> Spanned {
        Spanned { token, line, col }
    }

    fn next_token(&mut self) -> Result<Option<Spanned>, LexError> {
        // Skip whitespace and comments. We *do* emit newlines as tokens but
        // only one per run of blank lines.
        loop {
            match self.peek() {
                Some(b' ') | Some(b'\t') | Some(b'\r') => {
                    self.bump();
                }
                Some(b'/') if self.peek2() == Some(b'/') => {
                    while let Some(c) = self.peek() {
                        if c == b'\n' {
                            break;
                        }
                        self.bump();
                    }
                }
                _ => break,
            }
        }

        let line = self.line;
        let col = self.col;

        let c = match self.peek() {
            Some(c) => c,
            None => return Ok(None),
        };

        // Newline
        if c == b'\n' {
            self.bump();
            return Ok(Some(self.make(Token::Newline, line, col)));
        }

        // Byte string literal: b"..."
        if c == b'b' && self.peek2() == Some(b'"') {
            return Ok(Some(self.bytes_string(line, col)?));
        }

        // Identifiers and keywords
        if c.is_ascii_alphabetic() || c == b'_' {
            return Ok(Some(self.ident_or_keyword(line, col)));
        }

        // Numbers
        if c.is_ascii_digit() {
            return Ok(Some(self.number(line, col)?));
        }

        // Strings
        if c == b'"' {
            return Ok(Some(self.string(line, col)?));
        }

        // Punctuation and operators
        self.bump();
        let tok = match c {
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,
            b'[' => Token::LBracket,
            b']' => Token::RBracket,
            b',' => Token::Comma,
            b';' => Token::Semi,
            b':' => Token::Colon,
            b'.' => Token::Dot,
            b'?' => Token::Question,
            b'+' => Token::Plus,
            b'*' => Token::Star,
            b'/' => Token::Slash,
            b'%' => Token::Percent,
            b'-' => {
                if self.peek() == Some(b'>') {
                    self.bump();
                    Token::Arrow
                } else {
                    Token::Minus
                }
            }
            b'=' => {
                if self.peek() == Some(b'=') {
                    self.bump();
                    Token::Eq
                } else if self.peek() == Some(b'>') {
                    self.bump();
                    Token::FatArrow
                } else {
                    Token::Assign
                }
            }
            b'!' => {
                if self.peek() == Some(b'=') {
                    self.bump();
                    Token::NotEq
                } else {
                    Token::Not
                }
            }
            b'<' => {
                if self.peek() == Some(b'=') {
                    self.bump();
                    Token::LtEq
                } else {
                    Token::Lt
                }
            }
            b'>' => {
                if self.peek() == Some(b'=') {
                    self.bump();
                    Token::GtEq
                } else {
                    Token::Gt
                }
            }
            b'&' => {
                if self.peek() == Some(b'&') {
                    self.bump();
                    Token::And
                } else {
                    Token::Amp
                }
            }
            b'|' => {
                if self.peek() == Some(b'|') {
                    self.bump();
                    Token::Or
                } else {
                    return Err(LexError {
                        msg: "bare `|` not yet supported".into(),
                        line,
                        col,
                    });
                }
            }
            _ => {
                return Err(LexError {
                    msg: format!("unexpected character: {:?}", c as char),
                    line,
                    col,
                });
            }
        };

        Ok(Some(self.make(tok, line, col)))
    }

    fn ident_or_keyword(&mut self, line: usize, col: usize) -> Spanned {
        let start = self.pos;
        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == b'_' {
                self.bump();
            } else {
                break;
            }
        }
        let s = std::str::from_utf8(&self.src[start..self.pos])
            .expect("ASCII ident")
            .to_string();

        let tok = match s.as_str() {
            "fn" => Token::Fn,
            "let" => Token::Let,
            "mut" => Token::Mut,
            "move" => Token::Move,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "for" => Token::For,
            "in" => Token::In,
            "return" => Token::Return,
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),
            "nil" => Token::Nil,
            "panic" => Token::Panic,
            "async" => Token::Async,
            "await" => Token::Await,
            "spawn" => Token::Spawn,
            "join" => Token::Join,
            _ => Token::Ident(s),
        };

        self.make(tok, line, col)
    }

    fn number(&mut self, line: usize, col: usize) -> Result<Spanned, LexError> {
        let start = self.pos;
        let mut is_float = false;

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                self.bump();
            } else if c == b'.' && self.peek2().is_some_and(|n| n.is_ascii_digit()) {
                is_float = true;
                self.bump();
            } else {
                break;
            }
        }

        let s = std::str::from_utf8(&self.src[start..self.pos]).expect("ASCII num");
        let tok = if is_float {
            let v: f64 = s.parse().map_err(|_| LexError {
                msg: format!("invalid float: {}", s),
                line,
                col,
            })?;
            Token::Float(v)
        } else {
            let v: i64 = s.parse().map_err(|_| LexError {
                msg: format!("invalid int: {}", s),
                line,
                col,
            })?;
            Token::Int(v)
        };

        Ok(self.make(tok, line, col))
    }

    fn string(&mut self, line: usize, col: usize) -> Result<Spanned, LexError> {
        self.bump(); // consume opening "
        let mut s = String::new();
        loop {
            match self.peek() {
                None => {
                    return Err(LexError {
                        msg: "unterminated string".into(),
                        line,
                        col,
                    })
                }
                Some(b'"') => {
                    self.bump();
                    break;
                }
                Some(b'\\') => {
                    self.bump();
                    match self.bump() {
                        Some(b'n') => s.push('\n'),
                        Some(b't') => s.push('\t'),
                        Some(b'r') => s.push('\r'),
                        Some(b'\\') => s.push('\\'),
                        Some(b'"') => s.push('"'),
                        Some(other) => {
                            return Err(LexError {
                                msg: format!("bad escape: \\{}", other as char),
                                line: self.line,
                                col: self.col,
                            });
                        }
                        None => {
                            return Err(LexError {
                                msg: "unterminated escape".into(),
                                line,
                                col,
                            })
                        }
                    }
                }
                Some(c) if c < 0x80 => {
                    s.push(c as char);
                    self.bump();
                }
                Some(_) => {
                    // UTF-8 multi-byte sequence: consume the full codepoint
                    // so we don't split it into Latin-1 chars (which would
                    // double-encode on re-serialization).
                    let start = self.pos;
                    let width = utf8_width(self.src[start]);
                    if start + width > self.src.len() {
                        return Err(LexError {
                            msg: "invalid UTF-8 in string literal".into(),
                            line: self.line,
                            col: self.col,
                        });
                    }
                    let bytes = &self.src[start..start + width];
                    let ch = std::str::from_utf8(bytes)
                        .map_err(|_| LexError {
                            msg: "invalid UTF-8 in string literal".into(),
                            line: self.line,
                            col: self.col,
                        })?
                        .chars()
                        .next()
                        .unwrap();
                    s.push(ch);
                    // Advance position + column manually; multi-byte chars
                    // count as one column and never contain a newline.
                    self.pos += width;
                    self.col += 1;
                }
            }
        }
        Ok(self.make(Token::Str(s), line, col))
    }

    fn bytes_string(&mut self, line: usize, col: usize) -> Result<Spanned, LexError> {
        self.bump(); // consume b
        self.bump(); // consume opening "
        let mut bytes = Vec::new();
        loop {
            match self.peek() {
                None => {
                    return Err(LexError {
                        msg: "unterminated byte string".into(),
                        line,
                        col,
                    })
                }
                Some(b'"') => {
                    self.bump();
                    break;
                }
                Some(b'\\') => {
                    self.bump();
                    match self.bump() {
                        Some(b'n') => bytes.push(b'\n'),
                        Some(b't') => bytes.push(b'\t'),
                        Some(b'r') => bytes.push(b'\r'),
                        Some(b'\\') => bytes.push(b'\\'),
                        Some(b'"') => bytes.push(b'"'),
                        Some(b'x') => {
                            let hi = self.bump().ok_or_else(|| LexError {
                                msg: "unterminated hex byte escape".into(),
                                line,
                                col,
                            })?;
                            let lo = self.bump().ok_or_else(|| LexError {
                                msg: "unterminated hex byte escape".into(),
                                line,
                                col,
                            })?;
                            let h = hex_val(hi).ok_or_else(|| LexError {
                                msg: format!(
                                    "bad hex byte escape: \\x{}{}",
                                    hi as char, lo as char
                                ),
                                line,
                                col,
                            })?;
                            let l = hex_val(lo).ok_or_else(|| LexError {
                                msg: format!(
                                    "bad hex byte escape: \\x{}{}",
                                    hi as char, lo as char
                                ),
                                line,
                                col,
                            })?;
                            bytes.push((h << 4) | l);
                        }
                        Some(other) => {
                            return Err(LexError {
                                msg: format!("bad byte escape: \\{}", other as char),
                                line: self.line,
                                col: self.col,
                            });
                        }
                        None => {
                            return Err(LexError {
                                msg: "unterminated escape".into(),
                                line,
                                col,
                            })
                        }
                    }
                }
                Some(c) if c < 0x80 => {
                    bytes.push(c);
                    self.bump();
                }
                Some(_) => {
                    return Err(LexError {
                        msg: "non-ASCII byte string content must use \\xNN escapes".into(),
                        line: self.line,
                        col: self.col,
                    })
                }
            }
        }
        Ok(self.make(Token::Bytes(bytes), line, col))
    }
}

fn hex_val(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

fn utf8_width(first: u8) -> usize {
    if first < 0xc0 {
        1
    }
    // continuation byte as lead — caller will err
    else if first < 0xe0 {
        2
    } else if first < 0xf0 {
        3
    } else {
        4
    }
}
