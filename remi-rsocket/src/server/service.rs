use std::convert::Infallible;
use std::future::{ready, Ready};
use std::task::{Context, Poll};

use tower::Service;

use crate::instance::{Connection, RSocket};

impl<Conn> Service<Conn> for super::RSocketServer
where
    Conn: Into<Connection<Conn>>,
{
    type Error = Infallible;
    type Future = Ready<Result<Self::Response, Self::Error>>;
    type Response = RSocket<Conn>;

    #[inline]
    fn poll_ready(
        &mut self,
        _cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    #[inline]
    fn call(&mut self, req: Conn) -> Self::Future {
        let conn = RSocket::new(req.into());

        ready(Ok(conn))
    }
}
