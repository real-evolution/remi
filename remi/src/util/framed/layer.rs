use derive_new::new;
use tower::Layer;

use super::service::FramedStreamService;

#[derive(new)]
pub struct FramedStreamLayer<Codec> {
    codec: Codec,
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
