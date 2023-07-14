use std::future::ready;
use std::task::Poll;

use futures::future::BoxFuture;
use tower::Service;

impl<Conn, Svc> Service<Svc> for super::RSocket<Conn> {
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
