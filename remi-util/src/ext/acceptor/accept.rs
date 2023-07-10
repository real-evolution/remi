use std::pin::Pin;
use std::task::{Context, Poll};

use futures::Future;
use remi_core::io::Acceptor;

#[derive(Debug)]
pub struct Accept<'a, A> {
    acceptor: &'a mut A,
}

impl<'a, A: Acceptor> Accept<'a, A> {
    #[inline(always)]
    pub(super) fn new(acceptor: &'a mut A) -> Self {
        Self { acceptor }
    }
}

impl<'a, A: Acceptor + Unpin> Future for Accept<'a, A> {
    type Output = Result<A::Conn, A::Error>;

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Self::Output> {
        Pin::new(&mut *self.acceptor).poll_accept(cx)
    }
}
