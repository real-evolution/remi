use std::ops::{Deref, DerefMut};

use rsocket_proto::io::codec::FrameCodec;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_util::codec::Framed;

#[derive(Debug)]
pub struct Connection<T, C>(Framed<T, C>);

impl<T, C> Connection<T, C>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    #[inline]
    pub fn new(inner: T, codec: C) -> Self {
        Self(Framed::new(inner, codec))
    }

    #[inline]
    pub fn new_raw(inner: T) -> Connection<T, FrameCodec> {
        Connection::new(inner, FrameCodec::default())
    }
}

impl<T, C> Deref for Connection<T, C> {
    type Target = Framed<T, C>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, C> DerefMut for Connection<T, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
