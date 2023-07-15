mod io;
mod service;

use std::time::Duration;

use rsocket_proto::frame::Setup;

pub use self::io::{ext, pipe, StreamConnection};

#[derive(Debug)]
pub struct RSocket<Conn> {
    pub(crate) conn: Conn,
    pub(crate) lifetime: Duration,
    pub(crate) setup_frame: Setup,
}
