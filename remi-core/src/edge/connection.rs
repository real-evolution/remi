use tokio::io::{AsyncRead, AsyncWrite};

use crate::error::RemiResult;

#[derive(Debug, Clone)]
pub enum ConnectionState {
    Closed,
    Open,
}

#[crate::async_trait]
pub trait Connection: Sync {
    type Id: Clone + PartialEq + Eq + std::hash::Hash;

    /// Returns the id of this connection.
    fn id(&self) -> Self::Id;

    /// Returns the state of this connection.
    fn state(&self) -> ConnectionState;

    /// Closes the connection.
    async fn close(&self) -> RemiResult<()>;
}

/// A trait to represent a transport connection
#[crate::async_trait]
pub trait FramedConnection: Connection {
    type Frame: Send + Sync;

    /// Sends a frame through the connection.
    async fn send(&self, frame: Self::Frame) -> RemiResult<()>;

    /// Receives a frame from the connection.
    async fn next(&self) -> RemiResult<Self::Frame>;
}

/// A trait to represent a transport connection
#[crate::async_trait]
pub trait StreamConnection: Connection + AsyncRead + AsyncWrite {}
