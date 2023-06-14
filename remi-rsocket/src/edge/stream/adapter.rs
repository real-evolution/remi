use remi_core::{
    edge::{Connection, FramedConnection, StreamConnection},
    error::RemiResult,
};

use futures::{SinkExt, StreamExt};
use tokio_util::codec::Framed;

use super::{codec::RSocketStreamFrameCodec, frame::RSocketFrame};

pub struct RSocketStreamAdapter<C> {
    inner: Framed<C, RSocketStreamFrameCodec>,
}

#[remi::async_trait]
impl<C> Connection for RSocketStreamAdapter<C>
where
    C: Connection,
{
    type Id = <C as Connection>::Id;

    #[inline(always)]
    fn id(&self) -> Option<Self::Id> {
        self.inner.get_ref().id()
    }

    #[inline(always)]
    async fn close(self) -> RemiResult<()> {
        Ok(self.inner.into_inner().close().await?)
    }
}

#[remi::async_trait]
impl<C> FramedConnection for RSocketStreamAdapter<C>
where
    C: StreamConnection + Unpin,
{
    type Frame = RSocketFrame;

    #[inline(always)]
    async fn send(&mut self, frame: RSocketFrame) -> RemiResult<()> {
        Ok(self.inner.send(frame).await?)
    }

    #[inline(always)]
    async fn next(&mut self) -> Option<RemiResult<RSocketFrame>> {
        self.inner.next().await.map(|i| i.map_err(Into::into))
    }
}
