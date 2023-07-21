mod stream;
mod util;

/// A module to implement [`RConnection`] for raw byte-stream connections.
pub mod tagged;

use std::future::Future;

#[doc(inline)]
pub use self::stream::RStream;

/// A trait alias for the future returned by [`RConnection::serve_accept`].
pub trait ServeAcceptFuture<S> = Future<Output = crate::Result<Option<S>>>;

/// A trait to represent an rsocket-capable connection.
pub trait RConnection {
    /// The type of streams this connection can produce.
    type Stream: stream::RStream;

    /// The type of the future returned by `serve_accept`.
    type Future<'a>: ServeAcceptFuture<Self::Stream> + 'a
    where
        Self: 'a;

    /// Serves the connection until a new stream is established.
    ///
    /// This should be called in a loop, until it returns `None`.
    fn serve_accept(&mut self) -> Self::Future<'_>;
}
