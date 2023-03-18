use std::{fmt, io};

#[derive(Debug)]
pub enum Error {
	ServerBindingFailed { port: u16, io_error: io::Error },
	BadTlsParams(rustls::Error),
	ConnFailedToSendData(io::Error),
	ConnFailedToReceiveData(io::Error),
	ConnFailedToClose(io::Error)
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::ServerBindingFailed { port, io_error } => {
				writeln!(f, "Failed to bind server to port {port}: {io_error}")
			}
			Self::BadTlsParams(err) => writeln!(f, "Bad TLS parameters: {err}"),
			Self::ConnFailedToSendData(err) => writeln!(f, "Failed to send data to peer: {err}"),
			Self::ConnFailedToReceiveData(err) => {
				writeln!(f, "Failed to receive data from peer: {err}")
			}
			Self::ConnFailedToClose(err) => writeln!(f, "Failed to close connection: {err}")
		}
	}
}

impl std::error::Error for Error {}
