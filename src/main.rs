#![feature(async_closure)]

use config::{params, tls::read_tls_params};
use server::Server;
use tokio::io::AsyncWriteExt;

mod server;

mod config;

mod logger;

mod utils;

#[tokio::main]
async fn main() {
	logger::init();

	let port = params::port();
	let tls_params = return_on_err!(read_tls_params().await);

	let server = return_on_err!(Server::bind(port, tls_params).await);

	server
		.listen(async move |mut conn| {
			let addr = conn.peer_addr();
			println!("Connected with {addr}");

			conn.write_u8(69).await?;

			Ok(())
		})
		.await;
}
