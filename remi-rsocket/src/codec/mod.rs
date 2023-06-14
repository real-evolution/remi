mod frame;
mod adapter;

use bytes::BytesMut;
use remi_core::error::RemiError;
use rsocket_rust::utils::{u24, Writeable};
use tokio_util::codec::{Decoder, Encoder, LengthDelimitedCodec};

pub const RSOCKET_LEN_SIZE: usize = 3;

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct RSocketStreamFrameCodec(LengthDelimitedCodec);

impl RSocketStreamFrameCodec {
    pub fn new() -> Self {
        Self(
            LengthDelimitedCodec::builder()
                .big_endian()
                .length_field_length(RSOCKET_LEN_SIZE)
                .max_frame_length(u24::MAX as usize)
                .new_codec(),
        )
    }
}

impl Default for RSocketStreamFrameCodec {
    fn default() -> Self {
        Self::new()
    }
}

impl Decoder for RSocketStreamFrameCodec {
    type Item = frame::RSocketFrame;
    type Error = RemiError;

    fn decode(
        &mut self,
        src: &mut BytesMut,
    ) -> Result<Option<frame::RSocketFrame>, Self::Error> {
        let Some(mut buf) = self.0.decode(src)? else {
            return Ok(None);
        };

        rsocket_rust::frame::Frame::decode(&mut buf)
            .map(|f| Some(f.into()))
            .map_err(Self::Error::from)
    }
}

impl Encoder<frame::RSocketFrame> for RSocketStreamFrameCodec {
    type Error = RemiError;

    fn encode(
        &mut self,
        item: frame::RSocketFrame,
        dst: &mut BytesMut,
    ) -> Result<(), Self::Error> {
        dst.reserve(RSOCKET_LEN_SIZE + item.0.len());

        u24::from(item.0.len()).write_to(dst);
        item.0.write_to(dst);

        Ok(())
    }
}
