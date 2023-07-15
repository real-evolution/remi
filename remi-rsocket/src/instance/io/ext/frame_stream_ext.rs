use std::pin::Pin;
use std::task::{ready, Context, Poll};

use rsocket_proto::frame::Frame;

pub trait FrameStreamExt: super::FrameStream {
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

    fn poll_next_frame_unpin(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Frame, crate::Error>> {
        Pin::new(self).poll_next_frame(cx)
    }
}

impl<S> FrameStreamExt for S where S: super::FrameStream {}
