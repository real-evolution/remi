use futures::SinkExt;
use rsocket_proto::frame::{ErrorCode, Frame, StreamId};

pub trait FrameSinkExt: super::FrameSink {
    #[inline]
    fn send_error(
        &mut self,
        stream_id: StreamId,
        error_code: ErrorCode,
        message: &'static str,
    ) -> futures::sink::Send<'_, Self, Frame> {
        let error = Frame::builder()
            .stream_id(stream_id)
            .error()
            .code(error_code)
            .data(message.into())
            .build()
            .unwrap();

        self.send(error)
    }
}

impl<S: super::FrameSink> FrameSinkExt for S {}
