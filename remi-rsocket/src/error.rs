use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    /// An I/O error occurred. This is typically raised by the underlying
    /// transport.
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),

    /// A protocol error occurred. This is typically raised by the underlying
    /// framing protocol.
    #[error("rsocket protocol error: {0}")]
    Protocol(#[from] rsocket_proto::Error),

    /// An error occurred while pushing an item to a channel.
    #[error("channel produce error: {0}")]
    ChannelProduce(#[from] remi_util::channel::error::ProduceError),

    /// An error occurred while popping an item from a channel.
    #[error("channel consume error: {0}")]
    ChannelConsume(#[from] remi_util::channel::error::ConsumeError),

    /// An unexpected frame was received.
    #[error("unexpected frame `{}' (expected: `{}': {}", .actual_frame, .expected_frame, .message)]
    UnexpectedFrame {
        expected_frame: rsocket_proto::frame::FrameType,
        actual_frame: rsocket_proto::frame::FrameType,
        message: &'static str,
    },
}
