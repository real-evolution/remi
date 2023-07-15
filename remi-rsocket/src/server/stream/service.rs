use std::task::{Context, Poll};

use tokio::io::{AsyncRead, AsyncWrite};
use tower::Service;

use crate::instance::{RSocket, StreamConnection};
use crate::server::util;

impl<T> Service<T> for super::RSocketStreamServer
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    type Error = crate::Error;
    type Future = util::Setup<StreamConnection<T>>;
    type Response = RSocket<StreamConnection<T>>;

    #[inline]
    fn poll_ready(
        &mut self,
        _cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    #[inline]
    fn call(&mut self, conn: T) -> Self::Future {
        util::Setup::new(conn.into(), self.max_lifetime)
    }
}
