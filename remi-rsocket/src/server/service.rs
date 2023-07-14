use std::convert::Infallible;
use std::future::{ready, Ready};
use std::task::{Context, Poll};

use futures::future::BoxFuture;
use tower::Service;

use crate::io::Connection;
use crate::RSocket;

#[derive(Debug, Clone)]
pub struct RSocketServer;

impl<Conn> Service<Conn> for RSocketServer
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

impl<Conn, Svc> Service<Svc> for RSocket<Conn> {
    type Error = crate::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;
    type Response = ();

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _req: Svc) -> Self::Future {
        Box::pin(ready(Ok(())))
    }
}
