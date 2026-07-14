//! Local tmux-style `Ctrl+B`, `D` detach sequence filtering.

pub(super) struct Detector {
    prefix: bool,
}

pub(super) struct Filtered {
    pub data: Vec<u8>,
    pub detach: bool,
}

impl Detector {
    pub(super) fn new() -> Self {
        Self { prefix: false }
    }

    pub(super) fn filter(&mut self, bytes: &[u8]) -> Filtered {
        let mut data = Vec::with_capacity(bytes.len());
        for &byte in bytes {
            if self.prefix {
                self.prefix = false;
                if matches!(byte, b'd' | b'D') {
                    return Filtered { data, detach: true };
                }
                data.extend([2, byte]);
            } else if byte == 2 {
                self.prefix = true;
            } else {
                data.push(byte);
            }
        }
        Filtered {
            data,
            detach: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn detects_split_detach_sequence() {
        let mut value = Detector::new();
        assert!(!value.filter(&[2]).detach);
        assert!(value.filter(b"d").detach);
    }
    #[test]
    fn forwards_non_detach_prefix() {
        assert_eq!(Detector::new().filter(&[2, b'x']).data, vec![2, b'x']);
    }
}
