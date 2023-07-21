mod builder;
mod stream;

use futures::{Sink, SinkExt, Stream, TryStreamExt};
use rsocket_proto::frame::{Frame, StreamId, TaggedFrame};
use rsocket_proto::io::codec::{FragmentedFrameCodec, FrameCodec};
use tokio::sync::mpsc;
use tokio_util::codec::Framed;

pub use self::builder::TaggedConnectionBuilder;
pub use self::stream::TaggedStream;
use crate::conn::ServeAcceptFuture;

/// A type to represent a [`TaggedConnection`] with no support for
/// fragmentation.
pub type UnfragmentedConnection<T> = TaggedConnection<Framed<T, FrameCodec>>;

/// A type to represent a [`TaggedConnection`] that supports fragmentation of
/// frames from/into the size of the given `MTU`.
pub type FragmentedConnection<const MTU: usize, T> =
    TaggedConnection<Framed<T, FragmentedFrameCodec<MTU>>>;

/// A raw byte-stream connection adapter.
#[derive(Debug)]
pub struct TaggedConnection<C> {
    pub(super) inner: C,
    pub(super) mux_tx: rexer::Mux<StreamId, Frame>,
    pub(super) mux_rx: mpsc::Receiver<(StreamId, Frame)>,
}

impl<C, E> super::RConnection for TaggedConnection<C>
where
    C: Sink<TaggedFrame, Error = E>
        + Stream<Item = Result<TaggedFrame, E>>
        + Unpin,
    crate::Error: From<E>,
{
    type Stream = stream::TaggedStream;

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
