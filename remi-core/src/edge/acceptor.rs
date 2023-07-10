use std::task::{Context, Poll};

use super::connection::Connection;

/// An enum to represent the state of an acceptor.
#[derive(Debug, Default)]
pub enum AcceptorState {
    #[default]
    Stopped,
    Running,
}

/// A trait to represent a connection acceptor.
pub trait Acceptor: Unpin + Send {
    type Conn: Connection;
    type Error;

    // Polls for a new connection.
    fn poll_accept(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Self::Conn, Self::Error>>;

    /// Gets the current state of the acceptor.
    fn state(&self) -> AcceptorState;
}
