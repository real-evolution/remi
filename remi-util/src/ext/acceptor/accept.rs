use std::pin::Pin;
use std::task::{Context, Poll};

use futures::Future;
use remi_core::io::{Acceptor, AcceptorItem};

#[derive(Debug)]
pub struct Accept<'a, A>(&'a mut A);

impl<'a, A> Accept<'a, A> {
    #[inline(always)]
    pub(super) fn new(acceptor: &'a mut A) -> Self {
        Self(acceptor)
    }
}

impl<'a, A: Acceptor> Future for Accept<'a, A> {
    type Output = Result<AcceptorItem<A>, A::Error>;

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Self::Output> {
        self.0.poll_accept(cx)
    }
}
