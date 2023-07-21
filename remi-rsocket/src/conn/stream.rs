use derive_more::{From, Into};
use futures::{Sink, Stream};
use rsocket_proto::frame::{Frame, StreamId};

/// A type to represent an error that occurs when sending a frame.
#[derive(Debug, From, Into)]
pub struct FrameSendError(pub Option<(StreamId, Frame)>);

/// A type to represent a lane of a connection.
pub trait RStream {
    /// The type of the writer side of the lane.
    type Sink: Sink<Frame, Error = FrameSendError>;

    /// The type of the reader side of the lane.
    type Stream: Stream<Item = Frame>;

    /// Splits the lane into a sink and a stream.
    fn split(self) -> (Self::Sink, Self::Stream);
}
