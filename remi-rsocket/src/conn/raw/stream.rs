use rexer::util::lane::{LaneSink, LaneStream};
use rexer::Lane;
use rsocket_proto::frame::{Frame, StreamId};
use tokio_util::sync::PollSendError;

use crate::conn::stream::FrameSendError;
use crate::conn::util::SinkErrorAdapter;

/// A type to represent a stream of [`super::RawConnection`] connection.
#[derive(Debug)]
pub struct RawStream {
    tx: LaneSink<StreamId, Frame>,
    rx: LaneStream<StreamId, Frame>,
}

impl crate::conn::RStream for RawStream {
    type Sink = SinkErrorAdapter<LaneSink<StreamId, Frame>, FrameSendError>;
    type Stream = LaneStream<StreamId, Frame>;

    #[inline]
    fn split(self) -> (Self::Sink, Self::Stream) {
        let tx = SinkErrorAdapter::new(self.tx);
        let rx = self.rx;

        (tx, rx)
    }
}

impl From<Lane<StreamId, Frame>> for RawStream {
    fn from(value: Lane<StreamId, Frame>) -> Self {
        let (tx, rx) = value.split();

        Self {
            tx: tx.into(),
            rx: rx.into(),
        }
    }
}

impl From<PollSendError<(StreamId, Frame)>> for FrameSendError {
    #[inline]
    fn from(error: PollSendError<(StreamId, Frame)>) -> Self {
        Self(error.into_inner())
    }
}
