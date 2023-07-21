use futures::{SinkExt, TryStreamExt};
use rexer::lane::Lane;
use rsocket_proto::frame::{Frame, StreamId};
use rsocket_proto::io::codec::FrameCodec;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::mpsc;
use tokio_util::codec::Framed;

#[derive(Debug)]
pub struct StreamConnection<T> {
    inner: Framed<T, FrameCodec>,
    mux_tx: rexer::Mux<StreamId, Frame>,
    mux_rx: mpsc::Receiver<(StreamId, Frame)>,
}

impl<T> StreamConnection<T>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    #[inline]
    pub fn new(conn: T, conn_buf: usize, lane_buf: usize) -> Self {
        let inner = Framed::new(conn, FrameCodec::default());
        let (mux_tx, mux_rx) = rexer::Mux::new(conn_buf, lane_buf);

        Self {
            inner,
            mux_tx,
            mux_rx,
        }
    }

    async fn serve_accept(
        &mut self,
    ) -> crate::Result<Option<Lane<StreamId, Frame>>> {
        loop {
            tokio::select! {
                // incoming frames
                incoming = self.inner.try_next() => {
                    let (stream_id, frame) = match incoming? {
                        Some(tagged) => tagged.split(),
                        None => continue,
                    };

                    if let Some(lane) = self.mux_tx.send(stream_id, frame).await {
                        return Ok(Some(lane));
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
