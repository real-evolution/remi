use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),

    #[error("rsocket protocol error: {0}")]
    Protocol(#[from] rsocket_proto::Error),

    #[error("channel produce error: {0}")]
    ChannelProduce(#[from] remi_util::channel::error::ProduceError),

    #[error("channel consume error: {0}")]
    ChannelConsume(#[from] remi_util::channel::error::ConsumeError),
}
