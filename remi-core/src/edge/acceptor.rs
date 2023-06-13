use crate::error::RemiResult;

use super::connection::Connection;

/// A trait to represent a connection acceptor.
#[crate::async_trait]
pub trait Acceptor<C: Connection> {
    /// Accepts a new connection.
    async fn next(&self) -> RemiResult<C>;
}
