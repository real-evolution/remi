use crate::error::RemiResult;

use super::frame::Frame;

#[derive(Debug, Clone)]
pub enum ConnectionState {
    Closed,
    Open,
}

/// A trait to represent a transport connection
#[crate::async_trait]
pub trait Connection: Sync {
    type Id: Clone + PartialEq + Eq + std::hash::Hash;
    type Frame: Frame;

    /// Returns the id of this connection.
    fn id(&self) -> Self::Id;

    /// Returns the state of this connection.
    fn state(&self) -> ConnectionState;

    /// Sends a frame through the connection.
    async fn send(&self, frame: Self::Frame) -> RemiResult<()>;

    /// Receives a frame from the connection.
    async fn next(&self) -> RemiResult<Self::Frame>;

    /// Closes the connection.
    async fn close(&self) -> RemiResult<()>;
}
