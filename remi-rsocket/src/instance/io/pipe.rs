use futures::{Sink, Stream};
use rsocket_proto::frame::Frame;

pub trait FrameStream: Stream<Item = crate::Result<Frame>> + Unpin {}
pub trait FrameSink: Sink<Frame, Error = crate::Error> + Unpin {}
pub trait FramePipe: FrameStream + FrameSink {}

impl<T> FrameStream for T where T: Stream<Item = crate::Result<Frame>> + Unpin {}
impl<T> FrameSink for T where T: Sink<Frame, Error = crate::Error> + Unpin {}
impl<T> FramePipe for T where T: FrameStream + FrameSink {}
