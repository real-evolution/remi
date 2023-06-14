#[derive(Debug)]
#[repr(transparent)]
pub struct RSocketFrame(pub(crate) rsocket_rust::frame::Frame);

impl From<rsocket_rust::frame::Frame> for RSocketFrame {
    fn from(input: rsocket_rust::frame::Frame) -> Self {
        Self(input)
    }
}

impl remi_core::edge::Frame for RSocketFrame {}
