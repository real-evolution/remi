mod io;
mod service;

use std::time::Duration;

use rsocket_proto::frame::Setup;

pub use self::io::{ext, pipe, StreamConnection};

/// A server instance to handle a single connection.
#[derive(Debug)]
#[allow(dead_code)]
pub struct RSocket<Conn> {
    conn: Conn,
    lifetime: Duration,
    setup_frame: Setup,
}

impl<Conn> RSocket<Conn>
where
    Conn: pipe::FramePipe,
{
    #[inline]
    pub const fn new(
        conn: Conn,
        lifetime: Duration,
        setup_frame: Setup,
    ) -> Self {
        Self {
            conn,
            lifetime,
            setup_frame,
        }
    }
}
