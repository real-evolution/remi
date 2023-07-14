use bytes::BytesMut;
use rsocket_proto::frame::Frame;
use rsocket_proto::io::codec::FrameCodec;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_util::codec::{Decoder, Encoder, Framed};

#[derive(Debug)]
pub struct Connection<T>(Framed<T, FrameCodec>);

impl<T> Connection<T>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    #[inline]
    pub fn new(inner: T) -> Self {
        Self(Framed::new(inner, FrameCodec::default()))
    }
}

impl<T> Decoder for Connection<T> {
    type Error = crate::Error;
    type Item = Frame;

    #[inline]
    fn decode(
        &mut self,
        src: &mut BytesMut,
    ) -> Result<Option<Self::Item>, Self::Error> {
        self.0.codec_mut().decode(src).map_err(Into::into)
    }
}

impl<T> Encoder<Frame> for Connection<T> {
    type Error = crate::Error;

    #[inline]
    fn encode(
        &mut self,
        item: Frame,
        dst: &mut BytesMut,
    ) -> Result<(), Self::Error> {
        self.0.codec_mut().encode(item, dst).map_err(Into::into)
    }
}

impl<T, C> DerefMut for Connection<T, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
