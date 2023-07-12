use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConsumeError {
    #[error("consumer is closed")]
    Closed,

    #[error("producer is closed")]
    ProducerClosed,

    #[error("channel operation timeout reached")]
    Timeout,
}

#[derive(Debug, Error)]
pub enum ProduceError {
    #[error("channel is closed")]
    Closed,

    #[error("consumer is closed")]
    ConsumerClosed,
}

impl From<kanal::ReceiveError> for ConsumeError {
    fn from(value: kanal::ReceiveError) -> Self {
        match value {
            | kanal::ReceiveError::Closed => Self::Closed,
            | kanal::ReceiveError::SendClosed => Self::ProducerClosed,
        }
    }
}

impl From<kanal::SendError> for ProduceError {
    fn from(value: kanal::SendError) -> Self {
        match value {
            | kanal::SendError::Closed => Self::Closed,
            | kanal::SendError::ReceiveClosed => Self::ConsumerClosed,
        }
    }
}

impl From<kanal::SendErrorTimeout> for ProduceError {
    fn from(value: kanal::SendErrorTimeout) -> Self {
        match value {
            | kanal::SendErrorTimeout::Closed => Self::Closed,
            | kanal::SendErrorTimeout::ReceiveClosed => Self::ConsumerClosed,
            | kanal::SendErrorTimeout::Timeout => Self::ConsumerClosed,
        }
    }
}
