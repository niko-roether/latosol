use std::{
	future::Future,
	io::Write,
	net::{Shutdown, SocketAddr, TcpListener, TcpStream},
	sync::Arc
};

use log::{debug, error, info};
use rustls::{Certificate, PrivateKey, Reader, ServerConfig, ServerConnection, Writer};
use tokio::io;

use crate::errors::Error;

pub struct TlsParams {
	pub certificates: Vec<Certificate>,
	pub private_key: PrivateKey
}

fn create_tls_server_cfg(
	TlsParams {
		certificates,
		private_key
	}: TlsParams
) -> Result<ServerConfig, Error> {
	ServerConfig::builder()
		.with_safe_defaults()
		.with_no_client_auth()
		.with_single_cert(certificates, private_key)
		.map_err(Error::BadTlsParams)
}

#[derive(Debug)]
pub struct Connection {
	open: bool,
	socket: TcpStream,
	tls_conn: ServerConnection
}

impl Connection {
	fn new(socket: TcpStream, tls_conn: ServerConnection) -> Self {
		Self {
			open: true,
			socket,
			tls_conn
		}
	}

	pub fn reader(&mut self) -> RequestReader<'_> {
		RequestReader::new(self.tls_conn.reader())
	}

	pub fn receive(&mut self) -> Result<(), Error> {
		self.tls_conn
			.read_tls(&mut self.socket)
			.map_err(Error::ConnFailedToReceiveData)?;
		if self.tls_conn.process_new_packets().is_err() {
			self.close()?;
		}
		Ok(())
	}

	pub fn writer(&mut self) -> ResponseWriter<'_> {
		ResponseWriter::new(self.tls_conn.writer())
	}

	pub fn send(&mut self) -> Result<usize, Error> {
		self.tls_conn
			.write_tls(&mut self.socket)
			.map_err(Error::ConnFailedToSendData)
	}

	pub fn close(&mut self) -> Result<(), Error> {
		self.socket
			.shutdown(Shutdown::Both)
			.map_err(Error::ConnFailedToClose)
	}
}

pub struct ResponseWriter<'a> {
	writer: Writer<'a>
}

impl<'a> ResponseWriter<'a> {
	fn new(writer: Writer<'a>) -> Self {
		Self { writer }
	}
}

impl<'a> std::io::Write for ResponseWriter<'a> {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		self.writer.write(buf)
	}

	fn flush(&mut self) -> io::Result<()> {
		self.writer.flush()
	}
}

pub struct RequestReader<'a> {
	reader: Reader<'a>
}

impl<'a> RequestReader<'a> {
	fn new(reader: Reader<'a>) -> Self {
		Self { reader }
	}
}

impl<'a> std::io::Read for RequestReader<'a> {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		self.reader.read(buf)
	}
}

#[derive(Debug)]
pub struct Server {
	port: u16,
	listener: TcpListener,
	tls_config: Arc<ServerConfig>
}

impl Server {
	pub fn bind(port: u16, tls_params: TlsParams) -> Result<Self, Error> {
		let addr: SocketAddr = SocketAddr::new("::1".parse().unwrap(), port);
		let listener = TcpListener::bind(addr)
			.map_err(|io_error| Error::ServerBindingFailed { port, io_error })?;
		let tls_config = Arc::new(create_tls_server_cfg(tls_params)?);
		Ok(Self {
			port,
			listener,
			tls_config
		})
	}

	pub fn listen<H, F>(&self, handler: H)
	where
		H: (FnOnce(Connection) -> F) + Clone + Send + Sync + 'static,
		F: Future<Output = ()> + Send
	{
		info!("Server listening on port {}...", self.port);
		loop {
			match self.listener.accept() {
				Ok((socket, addr)) => {
					debug!("Established connection with {addr}");

					let tls_conn = ServerConnection::new(Arc::clone(&self.tls_config)).unwrap();
					let conn = Connection::new(socket, tls_conn);
					let handler = handler.clone();
					tokio::spawn(async move { handler(conn).await });
				}
				Err(err) => error!("Failed to establish incoming connection: {err}")
			}
		}
	}
}
