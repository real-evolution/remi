use tokio::io::{AsyncRead, AsyncWrite};

use crate::error::RemiResult;

/// A trait to represent a transport connection.
#[crate::async_trait]
pub trait Connection {
    type Id: Clone + PartialEq + Eq + std::hash::Hash;

    /// Returns the id of this connection.
    fn id(&self) -> Option<Self::Id>;
}

/// A trait to represent a frame-based transport connection.
#[crate::async_trait]
pub trait FramedConnection: Connection {
    type Frame: Send + Sync + Unpin;

    /// Sends a frame through the connection.
    async fn send(&mut self, frame: Self::Frame) -> RemiResult<()>;

    /// Receives a frame from the connection.
    async fn next(&mut self) -> Option<RemiResult<Self::Frame>>;
}

/// A trait to represent a stream-based transport connection.
#[crate::async_trait]
pub trait StreamConnection: Connection + AsyncRead + AsyncWrite {}
