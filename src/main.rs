#![feature(async_closure)]

use config::{
	params::{self, Parameters},
	tls::read_tls_params
};
use server::Server;
use tokio::io::AsyncWriteExt;

#[cfg(any(debug_assertions, test))]
use dotenvy::dotenv;

mod server;

mod config;

mod logger;

mod utils;

#[tokio::main]
async fn main() {
	#[cfg(any(debug_assertions, test))]
	dotenv().ok();

	logger::init();

	let Parameters { port } = params::read();
	let tls_params = fatal_error!(read_tls_params().await);
	let server = fatal_error!(Server::bind(port, tls_params).await);

	server
		.listen(async move |mut conn| {
			let addr = conn.peer_addr();
			println!("Connected with {addr}");

			conn.write_u8(69).await?;

			Ok(())
		})
		.await;
}
