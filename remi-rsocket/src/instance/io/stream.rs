use std::pin::Pin;
use std::task::{Context, Poll};

use futures::{Sink, SinkExt, Stream, StreamExt};
use rsocket_proto::frame::Frame;
use rsocket_proto::io::codec::FrameCodec;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_util::codec::Framed;

/// A stream transport connection. This is a thin wrapper around a
/// [`Framed`] transport, which can wrap any [`AsyncRead`] + [`AsyncWrite`]
/// implementing transport type.
#[derive(Debug)]
pub struct StreamConnection<T>(Framed<T, FrameCodec>);

impl<T> Stream for StreamConnection<T>
where
    T: AsyncRead + Unpin,
{
    type Item = crate::Result<Frame>;

    #[inline]
    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        self.0.poll_next_unpin(cx).map_err(Into::into)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<T> Sink<Frame> for StreamConnection<T>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    type Error = crate::Error;

    #[inline]
    fn poll_ready(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        self.0.poll_ready_unpin(cx).map_err(Into::into)
    }

    #[inline]
    fn start_send(
        mut self: Pin<&mut Self>,
        item: Frame,
    ) -> Result<(), Self::Error> {
        self.0.start_send_unpin(item).map_err(Into::into)
    }

    #[inline]
    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        self.0.poll_flush_unpin(cx).map_err(Into::into)
    }

    #[inline]
    fn poll_close(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        self.0.poll_close_unpin(cx).map_err(Into::into)
    }
}

impl<T> From<T> for StreamConnection<T>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    fn from(value: T) -> Self {
        Self(Framed::new(value, FrameCodec::default()))
    }
}
