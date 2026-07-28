pub(super) struct Cursor {
    chars: Vec<char>,
    pos: usize,
}

impl Cursor {
    pub(super) fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    pub(super) fn skip_ws(&mut self) {
        while self.peek_raw().is_some_and(char::is_whitespace) {
            self.pos += 1;
        }
    }

    pub(super) fn peek_raw(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    pub(super) fn bump_raw(&mut self) -> Option<char> {
        let ch = self.peek_raw()?;
        self.pos += 1;
        Some(ch)
    }

    pub(super) fn eat(&mut self, expected: char) -> bool {
        self.skip_ws();
        if self.peek_raw() == Some(expected) {
            self.pos += 1;
            true
        } else {
            false
        }
    }
}
