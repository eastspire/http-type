use crate::*;

/// Represents a decoded WebSocket frame
#[derive(Debug, Clone, Getter, Default, DisplayDebug)]
pub struct WebSocketFrame {
    /// FIN flag indicating if this is the final frame
    pub(super) fin: bool,
    /// Opcode indicating the frame type (text, binary, etc.)
    pub(super) opcode: u8,
    /// Mask flag indicating if the payload is masked
    pub(super) mask: bool,
    /// The payload data of the frame
    pub(super) payload_data: Vec<u8>,
}
