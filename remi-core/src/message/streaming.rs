use std::pin::Pin;
use std::task::{Context, Poll};

use futures::stream::BoxStream;
use futures::{Stream, StreamExt};

/// A wrapper type to represent a streaming message.
pub struct Streaming<'a, I> {
    stream: BoxStream<'a, I>,
}

impl<'a, I> Stream for Streaming<'a, I> {
    type Item = I;

    #[inline]
    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        self.stream.poll_next_unpin(cx)
    }
}
