use std::task::{Context, Poll};

use remi_core::edge::StreamConnection;
use tokio_util::codec::Framed;
use tower::Service;

use super::stream::FramedStreamConnection;

#[derive(Debug)]
pub struct FramedStreamService<S, Codec> {
    inner: S,
    codec: Codec,
}

impl<S, Codec> FramedStreamService<S, Codec> {
    #[inline]
    pub fn new(inner: S, codec: Codec) -> Self {
        Self { inner, codec }
    }
}

impl<S, Conn, Codec> Service<Conn> for FramedStreamService<S, Codec>
where
    S: Service<FramedStreamConnection<Conn, Codec>>,
    Conn: StreamConnection,
    Codec: Clone,
{
    type Error = S::Error;
    type Future = S::Future;
    type Response = S::Response;

    fn poll_ready(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Conn) -> Self::Future {
        self.inner.call(FramedStreamConnection::new(Framed::new(
            req,
            self.codec.clone(),
        )))
    }
}
