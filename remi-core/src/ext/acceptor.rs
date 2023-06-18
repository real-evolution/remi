use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::Future;

use crate::edge::Acceptor;

pub struct Accept<'a, A> {
    acceptor: &'a mut A,
}

impl<'a, A: Acceptor> Accept<'a, A> {
    #[inline(always)]
    fn new(acceptor: &'a mut A) -> Self {
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

#[async_trait::async_trait]
pub trait AcceptorExt: Acceptor + Sized {
    async fn accept(
        &mut self,
    ) -> Result<Option<<Self as Acceptor>::Conn>, <Self as Acceptor>::Error>
    {
        let conn = Accept::new(self).await?;

        Ok(Some(conn))
    }
}

impl<A: Acceptor> AcceptorExt for A {}
