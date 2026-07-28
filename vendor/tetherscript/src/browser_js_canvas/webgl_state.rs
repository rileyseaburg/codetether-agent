//! Deterministic WebGL metadata state.

pub(super) const MAX_COMMANDS: usize = 64;

#[derive(Clone)]
pub(super) struct WebGlState {
    pub version: u8,
    pub width: u32,
    pub height: u32,
    pub viewport: [i64; 4],
    pub clear_color: [f64; 4],
    pub commands: Vec<String>,
}

impl WebGlState {
    pub fn new(version: u8, width: u32, height: u32) -> Self {
        Self {
            version,
            width,
            height,
            viewport: [0, 0, width as i64, height as i64],
            clear_color: [0.0, 0.0, 0.0, 0.0],
            commands: Vec::new(),
        }
    }

    pub fn push(&mut self, command: String) {
        if self.commands.len() < MAX_COMMANDS {
            self.commands.push(command);
        }
    }
}
