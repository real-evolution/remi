use std::pin::Pin;
use std::task::{ready, Context, Poll};

use rsocket_proto::frame::Frame;

pub trait FrameStreamExt: super::FrameStream {
    /// Poll the next frame from the stream, returning an error if the stream
    /// is closed instead of returning `None`.
    fn poll_next_frame(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Frame, crate::Error>> {
        Poll::Ready(match ready!(self.poll_next(cx)) {
            | Some(frame) => Ok(frame?),
            | None => Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "connection closed",
            )
            .into()),
        })
    }

    /// A convenience method to call [`FrameStreamExt::poll_next_frame`] without
    /// needing to pin the stream.
    fn poll_next_frame_unpin(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Frame, crate::Error>> {
        Pin::new(self).poll_next_frame(cx)
    }
}

impl<S> FrameStreamExt for S where S: super::FrameStream {}
