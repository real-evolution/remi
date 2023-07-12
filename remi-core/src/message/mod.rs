pub mod codec;

pub trait Message {
    type Metadata;
    type Body;

    fn metadata(&self) -> &Self::Metadata;
    fn body(&self) -> &Self::Body;
    fn into_parts(self) -> (Self::Metadata, Self::Body);
}
