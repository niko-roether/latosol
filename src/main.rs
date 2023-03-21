#![feature(async_closure)]

use config::params::{self, Parameters};
use db::Database;
use server::Server;
use tokio::io::AsyncWriteExt;

#[cfg(any(debug_assertions, test))]
use dotenvy::dotenv;

mod server;

mod config;

mod logger;

mod utils;

mod db;

mod models;

#[tokio::main]
async fn main() {
	#[cfg(any(debug_assertions, test))]
	dotenv().ok();

	logger::init();

	let Parameters { port, db_max_conns } = params::read();

	let tls_params = fatal_error!(config::tls::read_params().await);
	let server = fatal_error!(Server::bind(port, tls_params).await);

	let db_location = fatal_error!(config::mongodb::read_location());
	let db = fatal_error!(Database::connect(db_location, db_max_conns).await);

	server
		.listen(async move |mut conn| {
			let addr = conn.peer_addr();
			println!("Connected with {addr}");

			conn.write_u8(69).await?;

			Ok(())
		})
		.await;
}
