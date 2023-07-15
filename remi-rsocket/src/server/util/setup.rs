use std::ops::{Deref, DerefMut};
use std::pin::{pin, Pin};
use std::task::{ready, Context, Poll};
use std::time::Duration;

use futures::{Future, SinkExt};
use rsocket_proto::frame::{ErrorCode, Frame, FrameType, FrameVariant};

use crate::instance::ext::FrameStreamExt;
use crate::instance::pipe::FramePipe;
use crate::instance::RSocket;

#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
#[pin_project::pin_project(project = SetupProj)]
pub enum Setup<Conn> {
    Handshake {
        conn: Option<Conn>,
        max_lifetime: Duration,
    },
    Error {
        conn: Conn,
        error_frame: Option<Frame>,
    },
}

impl<Conn> Setup<Conn> {
    #[inline]
    pub(crate) fn new(conn: Conn, max_lifetime: Duration) -> Self {
        Self::Handshake {
            conn: Some(conn),
            max_lifetime,
        }
    }
}

impl<Conn> Future for Setup<Conn>
where
    Conn: FramePipe,
{
    type Output = crate::Result<RSocket<Conn>>;

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Self::Output> {
        let this = self.as_mut().project();

        match this {
            | SetupProj::Handshake { conn, max_lifetime } => {
                let frame = conn
                    .as_mut()
                    .expect("polled `Setup' after completion")
                    .deref_mut()
                    .poll_next_frame_unpin(cx);

                let setup = match ready!(frame)?.split() {
                    | (_, FrameVariant::Setup(setup)) => setup,
                    | (header, _) => {
                        let error = Frame::builder()
                            .stream_id(header.stream_id())
                            .error()
                            .code(ErrorCode::InvalidSetup)
                            .data("first frame must be `SETUP'".into())
                            .build()?;

                        *self = Setup::Error {
                            conn: conn.take().unwrap(),
                            error_frame: Some(error),
                        };

                        return Poll::Pending;
                    }
                };

                let lifetime =
                    Duration::from_millis((*setup.lifetime().deref()) as u64)
                        .min(*max_lifetime);

                Poll::Ready(Ok(RSocket {
                    conn: conn.take().unwrap(),
                    lifetime,
                    setup_frame: setup,
                }))
            }
            | SetupProj::Error { conn, error_frame } => {
                ready!(conn.poll_ready_unpin(cx))?;

                if let Some(error_frame) = error_frame.take() {
                    conn.start_send_unpin(error_frame)?;
                }

                ready!(conn.poll_flush_unpin(cx))?;

                Poll::Ready(Err(crate::Error::UnexpectedFrame {
                    expected_frame: FrameType::Setup,
                    actual_frame: FrameType::Error,
                    message: "first frame must be `Setup`",
                }))
            }
        }
    }
}
