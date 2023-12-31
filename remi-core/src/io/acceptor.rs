use std::task::{Context, Poll};

/// A type alias for items returned by acceptors.
pub type AcceptorItem<A> =
    Accepted<<A as Acceptor>::Connection, <A as Acceptor>::Address>;

/// A trait to represent a connection acceptor.
pub trait Acceptor: Unpin + Send {
    type Connection;
    type Address;
    type Error;

    // Polls for a new connection.
    fn poll_accept(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Result<AcceptorItem<Self>, Self::Error>>;
}

/// A type to wrap an accepted connection with its address.
#[derive(Debug)]
pub struct Accepted<C, A> {
    conn: C,
    addr: A,
}

impl<C, A> Accepted<C, A> {
    /// Creates a new accepted item.
    ///
    /// # Parameters
    /// - `conn`: The accepted connection.
    /// - `addr`: The address of the accepted connection.
    #[inline]
    pub const fn new(conn: C, addr: A) -> Self {
        Self { conn, addr }
    }

    /// Deconstructs the accepted item into its connection and address pair.
    #[inline]
    pub fn split(self) -> (C, A) {
        (self.conn, self.addr)
    }

    /// Gets a reference to the inner connection.
    #[inline]
    pub const fn connection(&self) -> &C {
        &self.conn
    }

    /// Gets a reference to the inner address.
    #[inline]
    pub const fn address(&self) -> &A {
        &self.addr
    }
}

impl<C, A> super::Addressable for Accepted<C, A> {
    type Address = A;

    #[inline]
    fn address(&self) -> &Self::Address {
        &self.addr
    }
}

#[crate::async_trait]
impl<C, A> super::Connection for Accepted<C, A>
where
    C: super::Connection + Send,
    A: Send,
{
    type Error = C::Error;
    type Frame = C::Frame;

    async fn send(&mut self, frame: Self::Frame) -> Result<(), Self::Error> {
        self.conn.send(frame).await
    }

    async fn next(&mut self) -> Option<Result<Self::Frame, Self::Error>> {
        self.conn.next().await
    }
}
