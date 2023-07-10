use tower::Layer;

use super::service::FramedStreamService;

#[derive(Debug)]
pub struct FramedStreamLayer<Codec> {
    codec: Codec,
}

impl<Codec> FramedStreamLayer<Codec> {
    #[inline]
    pub fn new(codec: Codec) -> Self {
        Self { codec }
    }
}

impl<S, Codec> Layer<S> for FramedStreamLayer<Codec>
where
    Codec: Clone,
{
    type Service = FramedStreamService<S, Codec>;

    fn layer(&self, inner: S) -> Self::Service {
        FramedStreamService::new(inner, self.codec.clone())
    }
}
