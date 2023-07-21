use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

use derive_more::Into;
use futures::Sink;

pin_project_lite::pin_project! {
    /// A type to automatically convert a sink error to `E`.
    #[derive(Debug, Into)]
    pub struct SinkErrorAdapter<S, E> {
        #[pin]
        inner: S,
        _marker: PhantomData<E>,
    }
}

impl<S, E> SinkErrorAdapter<S, E> {
    /// Create a new [`SinkErrorAdapter<S, E>`] from a sink.
    #[inline(always)]
    pub fn new(inner: S) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }
}

impl<S, I, E> Sink<I> for SinkErrorAdapter<S, E>
where
    S: Sink<I> + Unpin,
    E: From<S::Error>,
{
    type Error = E;

    #[inline(always)]
    fn poll_ready(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        self.project().inner.poll_ready(cx).map_err(Into::into)
    }

    #[inline(always)]
    fn start_send(self: Pin<&mut Self>, item: I) -> Result<(), Self::Error> {
        self.project().inner.start_send(item).map_err(Into::into)
    }

    #[inline(always)]
    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        self.project().inner.poll_flush(cx).map_err(Into::into)
    }

    #[inline(always)]
    fn poll_close(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        self.project().inner.poll_close(cx).map_err(Into::into)
    }
}
