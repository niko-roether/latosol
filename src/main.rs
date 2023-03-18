#![feature(async_closure)]

use std::{fs::File, io::BufReader};

use env_logger::Env;
use rustls_pemfile::{certs, pkcs8_private_keys};
use server::{Server, TlsParams};
use tokio::io::AsyncWriteExt;
use tokio_rustls::rustls::{Certificate, PrivateKey};

mod server;

fn get_test_certs() -> Vec<Certificate> {
	let f = File::open("./test_crt/test.crt").unwrap();
	let mut reader = BufReader::new(f);
	certs(&mut reader)
		.unwrap()
		.into_iter()
		.map(Certificate)
		.collect()
}

fn get_test_key() -> PrivateKey {
	let f = File::open("./test_crt/test.key").unwrap();
	let mut reader = BufReader::new(f);
	let keys = pkcs8_private_keys(&mut reader).unwrap();
	PrivateKey(keys.into_iter().next().unwrap())
}

fn init_logger() {
	let env = Env::default()
		.filter("LATOSOL_LOG")
		.write_style("LATOSOL_LOG_STYLE")
		.default_filter_or("info");
	env_logger::init_from_env(env);
}

#[tokio::main]
async fn main() {
	init_logger();

	let server = Server::bind(
		6969,
		TlsParams {
			certificates: get_test_certs(),
			private_key: get_test_key()
		}
	)
	.await
	.unwrap();

	server
		.listen(async move |mut conn| {
			let addr = conn.peer_addr();
			println!("Connected with {addr}");

			conn.write_u8(69).await?;

			Ok(())
		})
		.await;
}
