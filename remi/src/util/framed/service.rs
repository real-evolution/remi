use derive_new::new;
use remi_core::edge::StreamConnection;

use std::task::{Context, Poll};
use tokio_util::codec::Framed;
use tower::Service;

use super::stream::FramedStreamConnection;

#[derive(new)]
pub struct FramedStreamService<S, Codec> {
    inner: S,
    codec: Codec,
}

impl<S, Conn, Codec> Service<Conn> for FramedStreamService<S, Codec>
where
    S: Service<FramedStreamConnection<Conn, Codec>>,
    Conn: StreamConnection,
    Codec: Clone,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        let x = 434;
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Conn) -> Self::Future {
        self.inner.call(FramedStreamConnection::new(Framed::new(
            req,
            self.codec.clone(),
        )))
    }
}
