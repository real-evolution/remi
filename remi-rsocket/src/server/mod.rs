use std::future::ready;
use std::marker::PhantomData;
use std::task::{Context, Poll};

use futures::future::BoxFuture;
use remi_core::server::protocol::Protocol;
use tokio::net::TcpStream;

#[derive(Debug, Clone, Default)]
pub struct RSocket<Conn> {
    _marker: PhantomData<Conn>,
}

impl RSocket<TcpStream> {
    #[inline]
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<Svc> Protocol<TcpStream, Svc> for RSocket<TcpStream> {
    type Error = crate::Error;
    type Future = BoxFuture<'static, Result<(), Self::Error>>;

    fn poll_ready(
        &mut self,
        _cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn serve(&mut self, conn: TcpStream, _service: Svc) -> Self::Future {
        println!("serving: {}", conn.local_addr().unwrap());

        Box::pin(ready(Ok(())))
    }
}
