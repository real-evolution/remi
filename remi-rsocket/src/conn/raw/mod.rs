mod builder;
mod stream;

use futures::{SinkExt, TryStreamExt};
use rsocket_proto::frame::{Frame, StreamId};
use rsocket_proto::io::codec::FrameCodec;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::mpsc;
use tokio_util::codec::Framed;

pub use self::builder::RawConnectionBuilder;
pub use self::stream::RawStream;

use crate::conn::ServeAcceptFuture;

/// A raw byte-stream connection adapter.
#[derive(Debug)]
pub struct RawConnection<T> {
    pub(super) inner: Framed<T, FrameCodec>,
    pub(super) mux_tx: rexer::Mux<StreamId, Frame>,
    pub(super) mux_rx: mpsc::Receiver<(StreamId, Frame)>,
}

impl<T> super::RConnection for RawConnection<T>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    type Stream = stream::RawStream;

    type Future<'a> = impl ServeAcceptFuture<Self::Stream> + 'a
    where
        Self: 'a;

    fn serve_accept(&mut self) -> Self::Future<'_> {
        async {
            loop {
                tokio::select! {
                    // incoming frames
                    incoming = self.inner.try_next() => {
                        let (stream_id, frame) = match incoming? {
                            Some(tagged) => tagged.split(),
                            None => continue,
                        };

                        if let Some(lane) = self.mux_tx.send(stream_id, frame).await {
                            return Ok(Some(lane.into()));
                        }
                    }

                    // outgoing frames
                    outgoing = self.mux_rx.recv() => {
                        let Some((stream_id, frame)) = outgoing else {
                            // TODO:
                            // Check the possiblity of this actually occurring.
                            return Ok(None);
                        };

                        self.inner.send(frame.tagged(stream_id)).await?;
                    }
                }
            }
        }
    }
}
