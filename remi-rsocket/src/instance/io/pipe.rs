use futures::{Sink, Stream};
use rsocket_proto::frame::Frame;

/// A trait to represent a stream of faillible frames. This is useful to
/// abstract away the underlying transport reading logic.
pub trait FrameStream: Stream<Item = crate::Result<Frame>> + Unpin {}

impl<T> FrameStream for T where T: Stream<Item = crate::Result<Frame>> + Unpin {}

/// A trait to represent a sink of faillible frames. This is useful to
/// abstract away the underlying transport writing logic.
pub trait FrameSink: Sink<Frame, Error = crate::Error> + Unpin {}

impl<T> FrameSink for T where T: Sink<Frame, Error = crate::Error> + Unpin {}

/// A convenience trait to represent a type with both [`FrameStream`] and
/// [`FrameSink`] implemented.
pub trait FramePipe: FrameStream + FrameSink {}

impl<T> FramePipe for T where T: FrameStream + FrameSink {}
