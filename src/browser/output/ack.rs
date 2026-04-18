use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Ack {
    pub ok: bool,
}
