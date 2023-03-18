use std::{future::Future, net::SocketAddr, pin::Pin, sync::Arc};

use anyhow::{Context, Result};
use tokio::{
	io::{AsyncRead, AsyncWrite},
	net::{TcpListener, TcpStream}
};
use tokio_rustls::{
	rustls::{Certificate, PrivateKey, ServerConfig},
	server::TlsStream,
	TlsAcceptor
};

pub struct TlsParams {
	pub certificates: Vec<Certificate>,
	pub private_key: PrivateKey
}

fn create_tls_server_cfg(
	TlsParams {
		certificates,
		private_key
	}: TlsParams
) -> Result<ServerConfig> {
	ServerConfig::builder()
		.with_safe_defaults()
		.with_no_client_auth()
		.with_single_cert(certificates, private_key)
		.context("Bad TLS parameters")
}

#[derive(Debug)]
pub struct Connection {
	peer_addr: SocketAddr,
	stream: TlsStream<TcpStream>
}

impl Connection {
	fn new(peer_addr: SocketAddr, stream: TlsStream<TcpStream>) -> Self {
		Self { peer_addr, stream }
	}

	pub fn peer_addr(&self) -> SocketAddr {
		self.peer_addr
	}
}

impl AsyncRead for Connection {
	fn poll_read(
		mut self: std::pin::Pin<&mut Self>,
		cx: &mut std::task::Context<'_>,
		buf: &mut tokio::io::ReadBuf<'_>
	) -> std::task::Poll<std::io::Result<()>> {
		Pin::new(&mut self.stream).poll_read(cx, buf)
	}
}

impl AsyncWrite for Connection {
	fn poll_write(
		mut self: Pin<&mut Self>,
		cx: &mut std::task::Context<'_>,
		buf: &[u8]
	) -> std::task::Poll<std::result::Result<usize, std::io::Error>> {
		Pin::new(&mut self.stream).poll_write(cx, buf)
	}

	fn poll_flush(
		mut self: Pin<&mut Self>,
		cx: &mut std::task::Context<'_>
	) -> std::task::Poll<std::result::Result<(), std::io::Error>> {
		Pin::new(&mut self.stream).poll_flush(cx)
	}

	fn poll_shutdown(
		mut self: Pin<&mut Self>,
		cx: &mut std::task::Context<'_>
	) -> std::task::Poll<std::result::Result<(), std::io::Error>> {
		Pin::new(&mut self.stream).poll_shutdown(cx)
	}
}

pub struct Server {
	port: u16,
	listener: TcpListener,
	acceptor: TlsAcceptor
}

impl Server {
	pub async fn bind(port: u16, tls_params: TlsParams) -> Result<Self> {
		let addr: SocketAddr = SocketAddr::new("::1".parse().unwrap(), port);
		let listener = TcpListener::bind(addr).await?;
		let tls_config = Arc::new(create_tls_server_cfg(tls_params)?);
		Ok(Self {
			port,
			listener,
			acceptor: TlsAcceptor::from(tls_config)
		})
	}

	pub async fn listen<H, F>(&self, handler: H)
	where
		H: (FnOnce(Connection) -> F) + Clone + Send + Sync + 'static,
		F: Future<Output = Result<()>> + Send
	{
		log::info!("Server listening on port {}...", self.port);

		loop {
			match self.listener.accept().await {
				Ok((stream, addr)) => self.accept_connection(stream, addr, handler.clone()).await,
				Err(err) => log::error!("Failed to establish an incoming connection: {err}")
			}
		}
	}

	async fn accept_connection<H, F>(&self, stream: TcpStream, addr: SocketAddr, handler: H)
	where
		H: (FnOnce(Connection) -> F) + Clone + Send + Sync + 'static,
		F: Future<Output = Result<()>> + Send
	{
		let acceptor = self.acceptor.clone();

		let future = async move {
			let stream = acceptor.accept(stream).await?;
			let connection = Connection::new(addr, stream);

			handler(connection).await?;

			Result::<()>::Ok(())
		};

		tokio::spawn(async move {
			if let Err(err) = future.await {
				log::error!("Connection with {addr} terminated: {err}")
			}
		});
	}
}
