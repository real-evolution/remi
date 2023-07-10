use futures::{SinkExt, StreamExt};
use remi_core::error::{RemiError, RemiResult};
use remi_core::io::{Connection, FramedConnection, StreamConnection};
use tokio_util::codec::{Decoder, Encoder, Framed};

#[derive(Debug)]
pub struct FramedStreamConnection<Conn, Codec> {
    inner: Framed<Conn, Codec>,
}

impl<Conn, Codec> FramedStreamConnection<Conn, Codec> {
    #[inline]
    pub fn new(inner: Framed<Conn, Codec>) -> Self {
        Self { inner }
    }

    #[inline(always)]
    pub fn into_inner(self) -> Framed<Conn, Codec> {
        self.inner
    }
}

#[remi_core::async_trait]
impl<Conn, Codec> Connection for FramedStreamConnection<Conn, Codec>
where
    Conn: Connection,
    Codec: Sync + Send,
{
    type Id = <Conn as Connection>::Id;

    #[inline(always)]
    fn id(&self) -> Option<Self::Id> {
        self.inner.get_ref().id()
    }

    #[inline(always)]
    async fn close(self) -> RemiResult<()> {
        Ok(self.inner.into_inner().close().await?)
    }
}

#[remi_core::async_trait]
impl<Conn, Codec> FramedConnection for FramedStreamConnection<Conn, Codec>
where
    Conn: StreamConnection + Unpin,
    Codec: Decoder + Encoder<Codec::Item> + Send + Sync,
    Codec::Item: Send + Sync + Unpin,
    RemiError: From<<Codec as Encoder<Codec::Item>>::Error>
        + From<<Codec as Decoder>::Error>,
{
    type Frame = Codec::Item;

    #[inline(always)]
    async fn send(&mut self, frame: Self::Frame) -> RemiResult<()> {
        Ok(self.inner.send(frame).await?)
    }

    #[inline(always)]
    async fn next(&mut self) -> Option<RemiResult<Self::Frame>> {
        self.inner.next().await.map(|i| i.map_err(Into::into))
    }
}
